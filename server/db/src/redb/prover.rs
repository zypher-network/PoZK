use ethers::types::Address;
use redb::TableDefinition;
use serde::{Deserialize, Serialize};

use crate::redb::{BaseTableDefinition, KvTable};

const PROVERS: BaseTableDefinition = TableDefinition::new("provers");

#[derive(Serialize, Deserialize)]
pub struct Prover {
    pub prover: Address,
    pub tag: String,
    pub image: String,
    pub name: String,
    pub overtime: u64,
    pub url: bool,
    pub created: i64,
}

impl Prover {
    pub fn to_key<'a>(prover: &'a Address) -> &'a [u8] {
        prover.as_bytes()
    }
}

impl KvTable for Prover {
    fn table<'a>() -> BaseTableDefinition<'a> {
        PROVERS
    }

    fn key(&self) -> Vec<u8> {
        Self::to_key(&self.prover).to_vec()
    }

    fn to_value(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or(vec![])
    }

    fn from_value(_key: &[u8], value: &[u8]) -> Option<Self> {
        serde_json::from_slice(value).ok()
    }
}
