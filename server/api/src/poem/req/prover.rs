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
pub struct ImagePullReq {
    pub repository: String,
    pub tag: String,
    pub name: String,
}
