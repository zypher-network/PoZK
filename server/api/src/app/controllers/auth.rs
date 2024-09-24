use axum::extract::{Extension, Json};
use serde_json::{json, Value};

use crate::app::extensions::auth::Erc4361Payload;
use crate::app::{AppContext, Result};

pub async fn index(Extension(app): Extension<AppContext>) -> String {
    format!("Hello, {:?}", app.miner)
}

pub async fn login(
    Extension(app): Extension<AppContext>,
    Json(payload): Json<Erc4361Payload>,
) -> Result<Json<Value>> {
    let token = payload
        .verify(&app.secret, &app.domains, &app.miner)
        .await?;

    Ok(Json(json!({ "token": token })))
}
