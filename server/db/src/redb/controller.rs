use ethers::{core::k256::ecdsa::SigningKey, types::Address};
use redb::TableDefinition;

use crate::redb::{BaseTableDefinition, KvTable};

const CONTROLLERS: BaseTableDefinition = TableDefinition::new("controllers");
const MAIN_CONTROLLER: BaseTableDefinition = TableDefinition::new("main_controller");
const MAIN_CONTROLLER_KEY: &str = "pozk_main_controller";

pub struct Controller {
    pub controller: Address,
    pub singing_key: SigningKey,
}

pub struct MainController {
    pub controller: Address,
}

impl Controller {
    pub fn to_key<'a>(controller: &'a Address) -> &'a [u8] {
        controller.as_bytes()
    }
}

impl MainController {
    pub fn to_key<'a>() -> &'a [u8] {
        MAIN_CONTROLLER_KEY.as_bytes()
    }
}

impl KvTable for Controller {
    fn table<'a>() -> BaseTableDefinition<'a> {
        CONTROLLERS
    }

    fn key(&self) -> Vec<u8> {
        Self::to_key(&self.controller).to_vec()
    }

    fn to_value(&self) -> Vec<u8> {
        self.singing_key.to_bytes().to_vec()
    }

    fn from_value(key: &[u8], value: &[u8]) -> Option<Self> {
        let controller = Address::from_slice(key);
        SigningKey::from_slice(value)
            .map(|singing_key| Self {
                controller,
                singing_key,
            })
            .ok()
    }
}

impl KvTable for MainController {
    fn table<'a>() -> BaseTableDefinition<'a> {
        MAIN_CONTROLLER
    }

    fn key(&self) -> Vec<u8> {
        Self::to_key().to_vec()
    }

    fn to_value(&self) -> Vec<u8> {
        self.controller.as_bytes().to_vec()
    }

    fn from_value(_key: &[u8], value: &[u8]) -> Option<Self> {
        Some(Self {
            controller: Address::from_slice(value),
        })
    }
}
