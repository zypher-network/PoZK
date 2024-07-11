use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::types::Address;
use ethers::utils::keccak256;
use redb::{Key, TableDefinition, TypeName, Value};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Debug;

pub const CONTROLLER_TABLE: TableDefinition<ControllerKey, ControllerValue> =
    TableDefinition::new("controller_table");

pub static CONTROLLER_SET_KEY: &str = "CONTROLLER_SET_KEY";
pub const CONTROLLER_SET: TableDefinition<&str, ControllerKey> =
    TableDefinition::new("controller_set");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControllerKey(pub Address);

impl From<&SigningKey> for ControllerKey {
    fn from(value: &SigningKey) -> Self {
        let pk = value.verifying_key().to_encoded_point(false);

        let pk_bytes = &pk.as_bytes()[1..];

        let hash = keccak256(pk_bytes);

        Self(Address::from_slice(&hash[12..]))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControllerValue(pub Vec<u8>);

impl From<&SigningKey> for ControllerValue {
    fn from(value: &SigningKey) -> Self {
        let data = value.to_bytes().to_vec();
        Self(data)
    }
}

impl Value for ControllerKey {
    type SelfType<'a> = ControllerKey where Self: 'a;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bincode::deserialize(data).unwrap_or(ControllerKey(Address::zero()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        bincode::serialize(value).unwrap_or_default()
    }

    fn type_name() -> TypeName {
        TypeName::new("ControllerKey")
    }
}

impl Key for ControllerKey {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        data1.cmp(data2)
    }
}

impl Value for ControllerValue {
    type SelfType<'a> = ControllerValue where Self: 'a;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bincode::deserialize(data).unwrap_or(ControllerValue(vec![]))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        bincode::serialize(value).unwrap_or_default()
    }

    fn type_name() -> TypeName {
        TypeName::new("ControllerValue")
    }
}
