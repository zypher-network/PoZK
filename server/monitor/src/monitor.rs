use crate::config::MonitorConfig;
use crate::event::{EventManager, EventType};
use anyhow::{anyhow, Result};
use chrono::Utc;
use ethers::abi::Token;
use ethers::prelude::{Http, Middleware, Provider, H256};
use ethers::types::{BlockNumber, Filter, U64};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// The CreateTask event on the listener chain is sent to the channel when the specified event is listened.
/// The event is processed by TxService.
/// Different events are processed by different channels.
/// In the future, you can consider using a map to store all channels.
#[derive(Clone)]
pub struct Monitor {
    eth_cli: Provider<Http>,
    cfg: MonitorConfig,
    filter: Filter,
    event_manager: EventManager,
    sender: Vec<UnboundedSender<MonitorChanData>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MonitorChanData {
    pub event_type: EventType,
    pub hash: Option<H256>,
    pub data: BTreeMap<String, Token>,
}

struct StartParam {
    from: u64,
    to: u64,
}

impl Monitor {
    pub async fn new(cfg: &MonitorConfig, eth_cli: Provider<Http>) -> Result<Self> {
        let event_manager =
            EventManager::new(&cfg.task_market_address, &cfg.prover_market_address)?;
        let filter = event_manager.get_filter()?;

        Ok(Self {
            eth_cli,
            cfg: cfg.clone(),
            filter,
            event_manager,
            sender: vec![],
        })
    }

    pub fn register(&mut self) -> UnboundedReceiver<MonitorChanData> {
        let (sender, receiver) = unbounded_channel();
        self.sender.push(sender);
        receiver
    }

    #[allow(unused_assignments)]
    pub fn run(self) {
        spawn(async move {
            let block_number_type = match BlockNumber::from_str(&self.cfg.block_number_type) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("[monitor] decode BlockNumber: {e:?}");
                    return;
                }
            };
            let (mut from, mut to) = {
                let start_param = match self.pares_from_and_to().await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[monitor] pares from and to: {e:?}");
                        return;
                    }
                };
                (start_param.from, start_param.to)
            };

            let mut filter = self.filter.clone();

            // Record the interval time and start time between each pull event
            let mut start_time = Utc::now().timestamp();

            'out: loop {
                let Some(Some(block)) = self.eth_cli.get_block(block_number_type).await.ok() else {
                    log::warn!("[monitor] get block is nil, type: {block_number_type:?}");
                    continue;
                };
                let Some(over_block_number) = block.number else {
                    log::warn!("[monitor] get block number is nil");
                    continue;
                };
                let over_block_number = over_block_number.as_u64();

                let step = self.cfg.step;

                // If to + step is greater than the latest height,
                // it sleeps until it is less than the latest height,
                // ensuring that the number of blocks of step is pulled each time
                // ps. If you pulled historical data before,
                // once you enter here, it proves that you have reached the latest height
                while to + step >= over_block_number {
                    tokio::time::sleep(Duration::from_secs(self.cfg.wait_time)).await;
                    continue 'out;
                }

                from = to + 1;
                to = to + step + 1;

                filter.block_option = filter
                    .block_option
                    .set_from_block(BlockNumber::Number(U64::from(from)))
                    .set_to_block(BlockNumber::Number(U64::from(to)));

                log::debug!(
                    "[monitor] from: {from}, to: {to}, {block_number_type:?}: {over_block_number}"
                );

                let logs = match self.eth_cli.get_logs(&filter).await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[monitor] get logs: {e:?}");
                        continue;
                    }
                };

                //Record the interval time and end time between each pull event
                let end_time = Utc::now().timestamp();

                log::debug!(
                    "[monitor] step: {step}, Duration: [{}]sec",
                    end_time - start_time
                );
                log::debug!("[monitor] logs size: {:?}", logs.len());

                for log in logs {
                    match self.event_manager.parse_log(&log) {
                        Ok(op) => {
                            if let Some(data) = op {
                                for (i, s) in self.sender.iter().enumerate() {
                                    if let Err(e) = s.send(data.clone()) {
                                        log::error!("[monitor] sender: [{i}], send data: {}", e.to_string());
                                    }
                                }
                                log::debug!("[monitor] sender all send success");
                            }
                        }
                        Err(e) => {
                            log::error!("[monitor] parse log: {e:?}");
                            continue;
                        }
                    }
                }

                // Whenever re-polling, the end time serves as the start time
                start_time = end_time;
            }
        });
    }

    async fn pares_from_and_to(&self) -> Result<StartParam> {
        let block_number =
            BlockNumber::from_str(&self.cfg.block_number_type).map_err(|e| anyhow!("{e}"))?;

        let block = self
            .eth_cli
            .get_block(block_number)
            .await?
            .ok_or(anyhow::Error::msg("get Finalized block is nil"))?;
        let finalized = block
            .number
            .ok_or("number is nil")
            .map_err(|e| anyhow!("err: {:?}", e))?;

        if finalized.is_zero() {
            return Err(anyhow::Error::msg("finalized is zero"));
        }

        let finalized = finalized.as_u64();

        let (from, to) = {
            if self.cfg.from == 0 {
                // latest
                (
                    finalized.saturating_sub(self.cfg.step * 2),
                    finalized.saturating_sub(self.cfg.step),
                )
            } else {
                // latest
                if self.cfg.from + self.cfg.step < finalized {
                    (self.cfg.from.saturating_sub(self.cfg.step), self.cfg.from)
                } else {
                    (
                        finalized.saturating_sub(self.cfg.step * 2),
                        finalized.saturating_sub(self.cfg.step),
                    )
                }
            }
        };

        Ok(StartParam {
            from: from.saturating_sub(1),
            to: to.saturating_sub(1),
        })
    }
}
