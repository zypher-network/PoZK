use axum::{
    body::Bytes,
    extract::{Extension, Json, Path},
};
use pozk_utils::{read_task_input, ServiceMessage};
use serde_json::Value;

use crate::app::{success, AppContext, Error, Result};

pub async fn download(Path(id): Path<String>) -> Result<Bytes> {
    let data = read_task_input(&id).await?;
    Ok(data.into())
}

pub async fn upload(
    Extension(app): Extension<AppContext>,
    Path(id): Path<String>,
    body: Bytes,
) -> Result<Json<Value>> {
    if body.is_empty() {
        return Err(Error::Invalid(1000, "Invalid proof length".to_owned()));
    }

    // send to task tx pool
    app.sender
        .send(ServiceMessage::UploadProof(id, body.to_vec()))
        .expect("Service sender invalid");

    Ok(success())
}
