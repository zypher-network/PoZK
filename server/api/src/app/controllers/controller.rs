use axum::extract::{Extension, Json, Path, Query};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{Address, LocalWallet, Signer};
use pozk_db::{Controller, MainController};
use pozk_utils::ServiceMessage;
use rand::thread_rng;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app::{success, AppContext, Error, Pagination, Result};

/// list all controllers
pub async fn index(
    Extension(app): Extension<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>> {
    let (begin, take_count) = pagination.begin_and_take();
    let (items, total) = app.db.list::<Controller>(begin, take_count)?;
    let data = items
        .iter()
        .map(|item| format!("{:?}", item.controller))
        .collect::<Vec<String>>();

    let m = app
        .db
        .get::<MainController>(MainController::to_key())?
        .map(|c| format!("{:?}", c.controller));

    Ok(Json(json!({
        "main": m,
        "data": data,
        "total": total
    })))
}

#[derive(Deserialize)]
pub struct CreateForm {
    signing_key: Option<String>,
}

/// generate/import a controller account
pub async fn create(
    Extension(app): Extension<AppContext>,
    Json(form): Json<CreateForm>,
) -> Result<Json<Value>> {
    let singing_key = if let Some(key) = form.signing_key {
        let bytes = hex::decode(key.trim_start_matches("0x"))
            .map_err(|_| Error::Invalid(1100, "Invalid secret key".to_owned()))?;
        SigningKey::from_slice(&bytes)
            .map_err(|_| Error::Invalid(1101, "Invalid secret key".to_owned()))?
    } else {
        SigningKey::random(&mut thread_rng())
    };

    let c = Controller {
        controller: LocalWallet::from(singing_key.clone()).address(),
        singing_key,
    };
    app.db.add(&c)?;

    Ok(Json(json!({
        "controller": format!("{:?}", c.controller),
        "singing_key": format!("0x{}", hex::encode(&c.singing_key.to_bytes()))
    })))
}

/// export controller account
pub async fn show(
    Extension(app): Extension<AppContext>,
    Path(address): Path<String>,
) -> Result<Json<Value>> {
    let address: Address = address
        .parse()
        .map_err(|_| Error::Invalid(1102, "Invalid address".to_owned()))?;

    let key = Controller::to_key(&address);
    let c = app
        .db
        .get::<Controller>(key)?
        .ok_or(Error::Invalid(1103, "Invalid address".to_owned()))?;

    Ok(Json(json!({
        "controller": address,
        "singing_key": format!("0x{}", hex::encode(&c.singing_key.to_bytes()))
    })))
}

/// set a controller as default signer
pub async fn update(
    Extension(app): Extension<AppContext>,
    Path(address): Path<String>,
) -> Result<Json<Value>> {
    let address: Address = address
        .parse()
        .map_err(|_| Error::Invalid(1102, "Invalid address".to_owned()))?;

    let key = Controller::to_key(&address);
    let c = app
        .db
        .get::<Controller>(key)?
        .ok_or(Error::Invalid(1103, "Invalid address".to_owned()))?;
    let sk_bytes = c.singing_key.to_bytes().as_slice().to_vec();

    app.sender
        .send(ServiceMessage::ChangeController(
            LocalWallet::from(c.singing_key),
            sk_bytes,
        ))
        .expect("Service sender invalid");

    Ok(success())
}
