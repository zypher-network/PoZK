use axum::{
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

/// Top level error type.
#[derive(Debug)]
pub enum Error {
    NotFound(i32),
    Internal(i32),
    Auth,
    Invalid(i32, String),
    Anyhow(String),
}

impl From<ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>>
    for Error
{
    fn from(
        err: ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>,
    ) -> Error {
        error!("ethers: {:?}", err);
        Error::Internal(2053)
    }
}

impl From<ethers::contract::AbiError> for Error {
    fn from(err: ethers::contract::AbiError) -> Error {
        error!("ethers ABI: {:?}", err);
        Error::Internal(2053)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_e: serde_json::Error) -> Error {
        Error::NotFound(2000)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Error {
        Error::Anyhow(e.to_string())
    }
}

impl From<()> for Error {
    fn from(_e: ()) -> Error {
        Error::Internal(2052)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (sc, code, msg) = match &self {
            Error::NotFound(i) => (StatusCode::NOT_FOUND, *i, "not found"),
            Error::Auth => (StatusCode::FORBIDDEN, 401, "auth invalid"),
            Error::Internal(i) => (StatusCode::INTERNAL_SERVER_ERROR, *i, "internal error"),
            Error::Invalid(i, e) => (StatusCode::BAD_REQUEST, *i, e.as_str()),
            Error::Anyhow(e) => (StatusCode::BAD_REQUEST, 400, e.as_str()),
        };

        let res = serde_json::to_string(&json!({ "code": code, "error": msg })).unwrap(); // safe
        (sc, res).into_response()
    }
}

pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
