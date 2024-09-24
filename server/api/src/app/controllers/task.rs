use axum::{
    body::Bytes,
    extract::{Extension, Json, Path},
};
use pozk_utils::{read_task_input, ServiceMessage};
use serde_json::Value;

use crate::app::{success, AppContext, Error, Result};

pub async fn download(Path(id): Path<u64>) -> Result<Bytes> {
    let data = read_task_input(id).await?;
    Ok(data.into())
}

pub async fn upload(
    Extension(app): Extension<AppContext>,
    Path(id): Path<u64>,
    body: Bytes,
) -> Result<Json<Value>> {
    if body.len() < 4 {
        return Err(Error::Invalid(1000, "Invalid proof length".to_owned()));
    }
    let mut output_len_bytes = [0u8; 4];
    output_len_bytes.copy_from_slice(&body[0..4]);
    let output_len = u32::from_be_bytes(output_len_bytes) as usize;
    if body.len() < output_len + 4 {
        return Err(Error::Invalid(1001, "Invalid proof length".to_owned()));
    }

    let raw_data = &body[4..];
    let output_bytes = raw_data[..output_len].to_vec();
    let proof_bytes = raw_data[output_len..].to_vec();

    // send to task tx pool
    app.sender
        .send(ServiceMessage::UploadProof(id, output_bytes, proof_bytes))
        .expect("Service sender invalid");

    Ok(success())
}
