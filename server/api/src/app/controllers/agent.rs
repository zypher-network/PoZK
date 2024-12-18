use axum::extract::{Extension, Json, Path, Query};
use ethers::prelude::Address;
use pozk_db::Prover;
use pozk_utils::{ProverType, ServiceMessage};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app::{success, AppContext, Error, Pagination, Result};

pub async fn index(Extension(app): Extension<AppContext>, body: String) -> Result<Json<Value>> {
    //
}

pub async fn run(Extension(app): Extension<AppContext>, body: String) -> Result<Json<Value>> {
    //
}

pub async fn log(Extension(app): Extension<AppContext>, Path(cid): Path<String>) -> Result<Json<Value>> {
    //
}
