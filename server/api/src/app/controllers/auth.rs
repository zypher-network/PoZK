use axum::{
    extract::{Extension, Json, Request, Path},
    response::{Html, IntoResponse, Redirect, Response},
    http::header::{HeaderValue, CONTENT_TYPE, HeaderMap}
};
use serde_json::{json, Value};
use tokio::fs::read;

use crate::app::extensions::auth::Erc4361Payload;
use crate::app::{AppContext, Result};

pub async fn login(
    Extension(app): Extension<AppContext>,
    Json(payload): Json<Erc4361Payload>,
) -> Result<Json<Value>> {
    let token = payload
        .verify(&app.secret, &app.domains, &app.miner)
        .await?;

    Ok(Json(json!({ "token": token })))
}

pub async fn webapp(req: Request) -> Response {
    let path = req.uri().path();
    let file_path = if path == "/" {
        "web-app/index.html".to_owned()
    } else {
        if path.contains(".") {
            format!("web-app{}", path)
        } else {
            format!("web-app{}.html", path)
        }
    };

    let header_value = mime_guess::from_path(&file_path)
        .first_raw()
        .map(HeaderValue::from_static)
        .unwrap_or(HeaderValue::from_str("application/octet-stream").unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, header_value);

    match read(&file_path).await {
        Ok(content) => {
            (headers, content).into_response()
        }
        Err(_) => {
            warn!("Invalid path: {}", file_path);
            Redirect::to("/").into_response()
        },
    }
}
