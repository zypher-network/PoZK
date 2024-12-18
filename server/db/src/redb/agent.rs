use ethers::types::Address;
use redb::TableDefinition;
use serde::{Deserialize, Serialize};

use crate::redb::{BaseTableDefinition, KvTable};

const AGENTS: BaseTableDefinition = TableDefinition::new("agents");

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub tid: u64,
    pub prover: Address,
    pub payer: Address,
    pub capacity: i32,
    pub endtime: i64,
    pub created: i64,
    pub is_me: bool,
    pub over: bool,
    pub container: String,
}

impl Agent {
    pub fn to_key(tid: u64) -> [u8; 8] {
        tid.to_le_bytes()
    }
}

impl KvTable for Agent {
    fn table<'a>() -> BaseTableDefinition<'a> {
        AGENTS
    }

    fn key(&self) -> Vec<u8> {
        Self::to_key(self.tid).to_vec()
    }

    fn to_value(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or(vec![])
    }

    fn from_value(_key: &[u8], value: &[u8]) -> Option<Self> {
        serde_json::from_slice(value).ok()
    }
}
