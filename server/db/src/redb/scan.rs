use redb::TableDefinition;

use crate::redb::{BaseTableDefinition, KvTable};

const SCAN_BLOCK: BaseTableDefinition = TableDefinition::new("scan_block");
const SCAN_BLOCK_KEY: &str = "pozk_scan_block";

pub struct ScanBlock {
    pub block: u64,
}

impl ScanBlock {
    pub fn to_key<'a>() -> &'a [u8] {
        SCAN_BLOCK_KEY.as_bytes()
    }
}

impl KvTable for ScanBlock {
    fn table<'a>() -> BaseTableDefinition<'a> {
        SCAN_BLOCK
    }

    fn key(&self) -> Vec<u8> {
        Self::to_key().to_vec()
    }

    fn to_value(&self) -> Vec<u8> {
        self.block.to_le_bytes().to_vec()
    }

    fn from_value(_key: &[u8], value: &[u8]) -> Option<Self> {
        let mut bytes = [0u8; 8];
        if value.len() == 8 {
            bytes.copy_from_slice(value);
        } else {
            return None;
        }

        Some(Self {
            block: u64::from_le_bytes(bytes),
        })
    }
}
