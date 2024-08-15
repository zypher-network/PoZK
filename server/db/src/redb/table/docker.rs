use crate::ControllerKey;
use ethers::types::Address;
use redb::{TableDefinition, TypeName, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// miner -> Map<repository, Vec<container_id>>
pub const DOCKER_TABLE: TableDefinition<ControllerKey, DockerValue> =
    TableDefinition::new("docker_table");

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DockerValue {
    /// prover-tag -> Vec<container_id>
    pub containers: BTreeMap<String, Vec<String>>,
    /// prover-tag -> (repository, tag, name, image_id)
    pub ids: BTreeMap<String, DockerImageMeta>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DockerImageMeta {
    pub prover: Address,
    pub repository: String,
    pub image_id: String,
    pub tag: String,
    pub name: String,
    pub created: String,
    pub overtime: u64,
}

impl Value for DockerValue {
    type SelfType<'a> = DockerValue where Self: 'a;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bincode::deserialize(data).unwrap_or(DockerValue {
            containers: Default::default(),
            ids: Default::default(),
        })
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        bincode::serialize(value).unwrap_or_default()
    }

    fn type_name() -> TypeName {
        TypeName::new("DockerValue")
    }
}
