use axum::extract::{Extension, Json, Path, Query};
use ethers::prelude::Address;
use pozk_db::Prover;
use pozk_utils::ServiceMessage;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app::{success, AppContext, Error, Pagination, Result};

/// list all provers in local
pub async fn index(
    Extension(app): Extension<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>> {
    let (begin, take_count) = pagination.begin_and_take();

    let (data, total) = app.db.list::<Prover>(begin, take_count)?;
    Ok(Json(json!({
        "data": data,
        "total": total,
    })))
}

#[derive(Deserialize)]
pub struct CreateForm {
    prover: String,
    tag: String,
    name: String,
    overtime: u64,
}

/// create & pull & running a prover
pub async fn create(
    Extension(app): Extension<AppContext>,
    Json(form): Json<CreateForm>,
) -> Result<Json<Value>> {
    let prover: Address = form
        .prover
        .parse()
        .map_err(|_| Error::Invalid(1102, "Invalid address".to_owned()))?;

    app.sender
        .send(ServiceMessage::PullProver(
            prover,
            form.tag,
            form.name,
            form.overtime,
        ))
        .expect("Service sender invalid");

    Ok(success())
}

/// show a prover detail
pub async fn show(
    Extension(app): Extension<AppContext>,
    Path(prover): Path<String>,
) -> Result<Json<Prover>> {
    let prover: Address = prover
        .parse()
        .map_err(|_| Error::Invalid(1102, "Invalid address".to_owned()))?;

    let key = Prover::to_key(&prover);
    let p = app
        .db
        .get::<Prover>(key)?
        .ok_or(Error::Invalid(1103, "Invalid address".to_owned()))?;
    Ok(Json(p))
}

/// delete a prover from local
pub async fn delete(
    Extension(app): Extension<AppContext>,
    Path(prover): Path<String>,
) -> Result<Json<Value>> {
    let prover: Address = prover
        .parse()
        .map_err(|_| Error::Invalid(1102, "Invalid address".to_owned()))?;

    app.sender
        .send(ServiceMessage::RemoveProver(prover))
        .expect("Service sender invalid");

    Ok(success())
}
