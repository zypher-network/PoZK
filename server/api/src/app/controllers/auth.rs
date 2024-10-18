use axum::{
    extract::{Extension, Json},
    response::Html,
};
use serde_json::{json, Value};
use std::{fs, net::SocketAddr, path::PathBuf};

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

pub async fn webapp(page: &str) -> Html<String> {
    let file_path = format!("web-app/{}.html", page); // 根據參數生成文件路徑

    // 讀取指定的 HTML 文件
    match tokio::fs::read_to_string(&file_path).await {
        Ok(content) => Html(content),
        Err(_) => Html("404 Not Found".to_string()), // 錯誤處理
    }
}
