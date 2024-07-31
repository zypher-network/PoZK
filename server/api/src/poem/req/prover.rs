use docker::ContainerNewOption;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ImagesUpdateReq {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerNewReq {
    pub option: ContainerNewOption,
    pub image: String,
    pub tag: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ProverPullReq {
    pub repository: String,
    pub prover: String,
    pub tag: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ProverNewReq {
    pub option: ContainerNewOption,
    pub prover: String,
    pub tag: String,
}
