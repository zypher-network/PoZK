use std::collections::BTreeMap;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use docker::ContainerNewOption;

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ImagesUpdateReq {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerNewReq{
    pub option: ContainerNewOption,
    pub image: String,
    pub tag: String,
}
