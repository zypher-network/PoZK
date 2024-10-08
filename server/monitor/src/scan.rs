use anyhow::{anyhow, Result};
use ethers::prelude::*;
use pozk_db::{ReDB, ScanBlock};
use pozk_utils::{new_providers, DefaultProvider, ServiceMessage};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::mpsc::UnboundedSender, time::timeout};

use crate::MonitorConfig;

const TIMEOUT: u64 = 10;

/// The CreateTask event on the listener chain is sent to the channel when the specified event is listened.
/// The event is processed by TxService.
/// Different events are processed by different channels.
/// In the future, you can consider using a map to store all channels.
#[derive(Clone)]
pub struct Scan {
    cfg: MonitorConfig,
    miner: Address,
    providers: Vec<Arc<DefaultProvider>>,
    init_start: Option<u64>,
    filter: Filter,
    events: HashMap<H256, EventType>,
    sender: UnboundedSender<ServiceMessage>,
    db: Arc<ReDB>,
}

#[derive(Clone)]
enum EventType {
    CreateTask,
    AcceptTask,
    ApproveProver,
}

#[derive(Clone, Debug, EthEvent)]
struct CreateTask {
    id: U256,
    prover: Address,
    player: Address,
    fee: U256,
    inputs: Bytes,
    publics: Bytes,
}

#[derive(Clone, Debug, EthEvent)]
struct AcceptTask {
    id: U256,
    miner: Address,
    overtime: U256,
}

#[derive(Clone, Debug, EthEvent)]
struct ApproveProver {
    prover: Address,
    work: U256,
    total: U256,
    epoch: U256,
    version: U256,
    overtime: U256,
    verifier: Address,
    minable: bool,
    approved: bool,
}

impl Scan {
    pub async fn new(
        cfg: MonitorConfig,
        sender: UnboundedSender<ServiceMessage>,
        db: Arc<ReDB>,
    ) -> Result<Self> {
        let providers = new_providers(&cfg.endpoints());
        let miner = cfg.miner()?;

        let (task_address, init_start) = cfg.task_address()?;
        let (prover_address, _) = cfg.prover_address()?;
        let addresses = vec![task_address, prover_address];

        let create_task = CreateTask::signature();
        let accept_task = AcceptTask::signature();
        let approve_prover = ApproveProver::signature();

        let mut events = HashMap::new();
        events.insert(create_task, EventType::CreateTask);
        events.insert(accept_task, EventType::AcceptTask);
        events.insert(approve_prover, EventType::ApproveProver);

        let topics = vec![create_task, accept_task, approve_prover];

        // filter
        let filter = Filter::new().address(addresses).topic0(topics);

        Ok(Self {
            cfg,
            miner,
            providers,
            init_start,
            filter,
            events,
            sender,
            db,
        })
    }

    pub fn run(self) {
        tokio::spawn(async move {
            let mut next_index = 0;
            let mut start_block = self.init_start.clone();

            if let Ok(Some(db_start)) = self.db.get::<ScanBlock>(ScanBlock::to_key()) {
                if let Some(init_start) = &self.init_start {
                    if *init_start < db_start.block {
                        start_block = Some(db_start.block);
                    }
                } else {
                    start_block = Some(db_start.block);
                }
            }

            loop {
                let start = if start_block.is_some() {
                    start_block
                } else {
                    if let Ok(start_block) = self.providers[next_index].get_block_number().await {
                        Some(start_block.as_u64() - self.cfg.delay)
                    } else {
                        None
                    }
                };

                if let Some(start) = start {
                    start_block = Some(self.running(start, next_index).await);
                }

                // waiting 2s
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                next_index += 1;
                if next_index == self.providers.len() {
                    next_index = 0;
                }
                error!("[Scan] provider failure, next_index: {}", next_index);
            }
        });
    }

    /// Loop running scan task
    pub async fn running(&self, mut start: u64, i: usize) -> u64 {
        loop {
            let start_time = Instant::now();

            let end_res = if let Ok(res) = timeout(
                Duration::from_secs(TIMEOUT),
                self.providers[i].get_block_number(),
            )
            .await
            {
                res
            } else {
                warn!("[Scan] Timeout: {}", i);
                return start;
            };
            if let Err(err) = end_res {
                error!("[Scan] Provider: {}", err);
                return start;
            }
            let mut end = end_res.unwrap().as_u64() - self.cfg.delay; // safe
            if start == end {
                debug!("[Scan] no new block: {}", start);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }
            if end > start && end - start > self.cfg.step {
                end = start + self.cfg.step;
            }

            let (from, to) = if start > end {
                (end, start)
            } else {
                (start + 1, end)
            };

            let new_filter = self.filter.clone().from_block(from).to_block(to);

            let logs = match self.providers[i].get_logs(&new_filter).await {
                Ok(v) => v,
                Err(e) => {
                    error!("[Scan] get logs: {e:?}");
                    continue;
                }
            };

            for log in logs {
                match self.parse_log(log) {
                    Ok(op) => {
                        self.sender.send(op).expect("Missing scan receiver"); // panic if channel is missing
                    }
                    Err(e) => {
                        error!("[Scan] parse log: {e:?}");
                        continue;
                    }
                }
            }

            info!(
                "[Scan] {start} - {end}, Duration: [{}]sec",
                start_time.elapsed().as_secs()
            );

            start = end;

            let _ = self.db.add(&ScanBlock { block: start });

            // waiting 2s
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    fn parse_log(&self, log: Log) -> Result<ServiceMessage> {
        let topic = &log.topics[0];
        if let Some(et) = self.events.get(topic) {
            match et {
                EventType::CreateTask => {
                    let ct = <CreateTask as EthEvent>::decode_log(&log.into())?;
                    let tid = ct.id.as_u64();
                    info!("[Scan] fetch new CreateTask: {}", tid);
                    Ok(ServiceMessage::CreateTask(tid, ct.prover, ct.inputs.to_vec(), ct.publics.to_vec()))
                }
                EventType::AcceptTask => {
                    let at = <AcceptTask as EthEvent>::decode_log(&log.into())?;
                    let is_me = at.miner == self.miner;
                    info!("[Scan] fetch new AcceptTask: {}", is_me);
                    Ok(ServiceMessage::AcceptTask(at.id.as_u64(), is_me))
                }
                EventType::ApproveProver => {
                    let ap = <ApproveProver as EthEvent>::decode_log(&log.into())?;
                    let version = ap.version.as_u64();
                    let overtime = ap.overtime.as_u64();
                    info!(
                        "[Scan] fetch new ApproveProver: {} - {}",
                        ap.prover, version
                    );
                    Ok(ServiceMessage::ApproveProver(ap.prover, version, overtime))
                }
            }
        } else {
            Err(anyhow!("missing topic"))
        }
    }
}
