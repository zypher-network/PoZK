use axum::{
    body::Bytes,
    extract::{Extension, Json, Path},
};
use chrono::Utc;
use ethers::prelude::{Address, Signature, H160};
use pozk_db::Prover;
use pozk_docker::RunOption;
use pozk_utils::{
    check_task_proxy_list, read_task_input, read_task_proof, write_task_input, ServiceMessage,
};
use rand::distributions::{Alphanumeric, DistString};
use serde_json::{json, Value};

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

/// receive tasks from player service
pub async fn create(Extension(app): Extension<AppContext>, body: Bytes) -> Result<Json<Value>> {
    let task = parse_task(body)?;

    // check signer
    if !check_task_proxy_list(&task.signer) {
        let res: bool = app.task.proxy_list(task.signer).await?;
        if !res {
            return Err(Error::Invalid(2005, "Invalid signer".to_owned()));
        }
    }

    // 1. check prover
    let key = Prover::to_key(&task.prover);
    let p = app
        .db
        .get::<Prover>(key)?
        .ok_or(Error::Invalid(1103, "Invalid prover".to_owned()))?;

    // 2. write data to file
    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
    let sid = format!("p-{}", code);
    write_task_input(&sid, task.inputs, task.publics).await?;

    // 3. start docker container to run, TODO we can do more about cpu & memory
    let _container = app.docker.run(&p.image, &sid, RunOption::default()).await?;

    // 4. create one time channel to services
    let over_at = Utc::now().timestamp() + p.overtime as i64;
    if app
        .sender
        .send(ServiceMessage::ApiTask(sid.clone(), over_at))
        .is_err()
    {
        return Err(Error::Internal(2002));
    }

    Ok(Json(json!({
        "code": 0,
        "track": sid,
        "overtime": over_at,
    })))
}

/// track task result from player service
pub async fn track(Path(id): Path<String>) -> Result<Json<Value>> {
    match read_task_proof(&id).await {
        Ok(proof) => Ok(Json(json!({
            "code": 0,
            "result": hex::encode(proof),
        }))),
        Err(_) => Err(Error::NotFound(2004)),
    }
}

struct ServiceTask {
    signer: Address,
    prover: Address,
    inputs: Vec<u8>,
    publics: Vec<u8>,
}

fn parse_task(bytes: Bytes) -> Result<ServiceTask> {
    if bytes.len() < 89 {
        return Err(Error::Invalid(2000, "Invalid task length".to_owned()));
    }

    let signature = Signature::try_from(&bytes[0..65])
        .map_err(|_| Error::Invalid(2006, "Invalid signature".to_owned()))?;
    let signer = signature
        .recover(&bytes[65..])
        .map_err(|_| Error::Invalid(2007, "Invalid signature".to_owned()))?;

    let mut prover_bytes = [0u8; 20];
    prover_bytes.copy_from_slice(&bytes[65..85]);
    let prover = H160(prover_bytes);

    let mut inputs_len_bytes = [0u8; 4];
    inputs_len_bytes.copy_from_slice(&bytes[85..89]);
    let inputs_len = u32::from_be_bytes(inputs_len_bytes) as usize;
    if bytes.len() < 89 + inputs_len {
        return Err(Error::Invalid(2001, "Invalid task length".to_owned()));
    }

    let inputs = bytes[89..89 + inputs_len].to_vec();
    let publics = bytes[89 + inputs_len..].to_vec();

    Ok(ServiceTask {
        signer,
        prover,
        inputs,
        publics,
    })
}
