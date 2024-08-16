use crate::monitor::MonitorChanData;
use crate::PROVER_MARKET_CONTRACT_ABI;
use crate::TASK_MARKET_CONTRACT_ABI;
use anyhow::Result;
use ethers::abi::{Bytes as AbiBytes, RawLog};
use ethers::abi::{Contract, Event};
use ethers::prelude::ValueOrArray;
use ethers::types::{Address, Filter, Log, H256};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EventType {
    CreateTask,
    ApproveProver,
}

/// Event management module, events that need to be processed are stored and classified here.
/// Listeners do not need to process events,
/// they only need to throw events to the event management module.
#[derive(Clone)]
pub struct EventManager {
    address_contracts: BTreeMap<Address, Contract>,
    topic_event: BTreeMap<H256, (Event, EventType)>,
}

impl EventManager {
    pub fn new(task_market_address: &str, prover_market_address: &str) -> Result<Self> {
        let mut topic_event = BTreeMap::new();
        let mut address_contracts = BTreeMap::new();

        let _insert_task = {
            let task_market = serde_json::from_str::<Contract>(TASK_MARKET_CONTRACT_ABI)?;
            let event = task_market.event("CreateTask")?;
            let topic = event.signature();

            topic_event.insert(topic, (event.clone(), EventType::CreateTask));

            let address = Address::from_str(task_market_address)?;
            address_contracts.insert(address, task_market);
        };

        let _insert_prover = {
            let prover_market = serde_json::from_str::<Contract>(PROVER_MARKET_CONTRACT_ABI)?;
            let event = prover_market.event("ApproveProver")?;
            let topic = event.signature();

            topic_event.insert(topic, (event.clone(), EventType::ApproveProver));

            let address = Address::from_str(prover_market_address)?;
            address_contracts.insert(address, prover_market);
        };

        Ok(Self {
            topic_event,
            address_contracts,
        })
    }

    pub fn parse_log(&self, log: &Log) -> Result<Option<MonitorChanData>> {
        let topic = &log.topics[0];
        if let Some((event, ty)) = self.topic_event.get(topic) {
            let raw_log = RawLog {
                topics: log.topics.clone(),
                data: AbiBytes::from(log.data.to_vec()),
            };
            let abilog = event.parse_log(raw_log)?;
            let map = abilog
                .params
                .into_iter()
                .map(|v| (v.name, v.value))
                .collect::<BTreeMap<_, _>>();

            log::debug!(
                "topic: {topic:?}, tx_hash: {:?}, event_type: {ty:?}",
                log.transaction_hash
            );

            return Ok(Some(MonitorChanData {
                event_type: ty.clone(),
                hash: log.transaction_hash,
                data: map,
            }));
        }

        Ok(None)
    }

    pub fn get_filter(&self) -> Result<Filter> {
        let address_list = self
            .address_contracts
            .iter()
            .map(|(addr, _)| addr.clone())
            .collect::<Vec<_>>();
        let topic_list = self
            .topic_event
            .iter()
            .map(|(topic, _)| Some(topic.clone()))
            .collect::<Vec<_>>();

        let mut filter = Filter::default();
        filter.address = Some(ValueOrArray::Array(address_list));
        filter.topics = [Some(ValueOrArray::Array(topic_list)), None, None, None];

        Ok(filter)
    }
}
