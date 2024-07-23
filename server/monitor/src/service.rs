use crate::event::EventManager;
use crate::tx::TxChanData;
use crate::MonitorConfig;
use anyhow::{anyhow, Result};
use db::ReDB;
use ethers::prelude::{Http, Middleware, Provider, ProviderExt};
use ethers::types::{BlockNumber, Filter, U64};
use std::str::FromStr;
use std::sync::Arc;
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
    tx_sender: Option<UnboundedSender<TxChanData>>,
}

struct StartParam {
    from: u64,
    to: u64,
}

impl Monitor {
    pub async fn new(cfg: &MonitorConfig, eth_cli: Provider<Http>) -> Result<Self> {
        let event_manager = EventManager::new(&cfg.task_market_address)?;
        let filter = event_manager.get_filter()?;

        Ok(Self {
            eth_cli,
            cfg: cfg.clone(),
            filter,
            event_manager,
            tx_sender: None,
        })
    }

    pub fn register_tx_sender(&mut self) -> UnboundedReceiver<TxChanData> {
        let (sender, receiver) = unbounded_channel();
        self.tx_sender.replace(sender);
        receiver
    }

    pub fn run(self) {
        spawn(async move {
            let block_number = match BlockNumber::from_str(&self.cfg.block_number_type) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("decode BlockNumber err: {e:?}");
                    return;
                }
            };
            let (mut from, mut to) = {
                let start_param = match self.pares_from_and_to().await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("monitor run err: {e:?}");
                        return;
                    }
                };
                (start_param.from, start_param.to)
            };

            let mut filter = self.filter.clone();

            'out: loop {
                let Some(Some(block)) = self.eth_cli.get_block(block_number).await.ok() else {
                    log::warn!("get block is nil, type: {block_number:?}");
                    continue;
                };
                let Some(finalized) = block.number else {
                    log::warn!("get block number is nil");
                    continue;
                };
                let finalized = finalized.as_u64();

                let step = self.cfg.step;

                // If to + step is greater than the latest height,
                // it sleeps until it is less than the latest height,
                // ensuring that the number of blocks of step is pulled each time
                // ps. If you pulled historical data before,
                // once you enter here, it proves that you have reached the latest height
                log::debug!("from: {from}, to: {to}, finalized: {finalized}");
                while to + step >= finalized {
                    tokio::time::sleep(Duration::from_secs(self.cfg.wait_time)).await;
                    continue 'out;
                }

                from = to + 1;
                to = to + step + 1;

                filter.block_option = filter
                    .block_option
                    .set_from_block(BlockNumber::Number(U64::from(from)))
                    .set_to_block(BlockNumber::Number(U64::from(to)));

                let logs = match self.eth_cli.get_logs(&filter).await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("monitor run: get logs err: {e:?}");
                        continue;
                    }
                };

                log::debug!("los: {logs:?}");

                for log in logs {
                    match self.event_manager.parse_log(&log) {
                        Ok(op) => {
                            if let Some(data) = op {
                                if let Some(sender) = self.tx_sender.clone() {
                                    if let Err(e) = sender.send(data) {
                                        log::error!("send data err: {e:?}");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("parse log err: {e:?}");
                            continue;
                        }
                    }
                }
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
                    (
                        self.cfg.from.saturating_sub(self.cfg.step),
                        self.cfg.from,
                    )
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

#[cfg(test)]
mod test {
    use crate::service::Monitor;
    use crate::MonitorConfig;
    use ethers::prelude::{Provider, ProviderExt};

    #[test]
    fn test_monitor() {
        env_logger::init();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let opbnb_testnet_cli = Provider::connect("http://127.0.0.1:8545").await;
            let cfg = MonitorConfig {
                task_market_address: "".to_string(),
                prover_market_address: "".to_string(),
                stake_address: "".to_string(),
                from: 0,
                delay_sec: 0,
                step: 0,
                wait_time: 0,
                block_number_type: "34736669".to_string(),
                miner: "".to_string(),
            };

            let mut monitor = Monitor::new(&cfg, opbnb_testnet_cli.clone()).await.unwrap();

            monitor.run();

            tokio::signal::ctrl_c().await.unwrap();
        });
    }
}
