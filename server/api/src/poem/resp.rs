use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct RespData {
    pub data: Option<Value>,
    pub msg: Option<String>,
    pub code: i32,
    pub uid: String,
}

impl RespData {
    pub fn new_data(data: &Value, uid: &str) -> Self {
        Self {
            data: Some(data.clone()),
            msg: None,
            code: 0,
            uid: uid.to_string(),
        }
    }

    pub fn new(uid: &str) -> Self {
        Self {
            data: None,
            msg: None,
            code: 0,
            uid: uid.to_string(),
        }
    }

    pub fn new_msg(msg: String, uid: &str, code: i32) -> Self {
        Self {
            data: None,
            msg: Some(msg),
            code,
            uid: uid.to_string(),
        }
    }

    pub fn new_err(msg: String, uid: &str) -> Self {
        Self {
            data: None,
            msg: Some(msg),
            code: -1,
            uid: uid.to_string(),
        }
    }
}

#[derive(ApiResponse)]
pub enum Resp {
    #[oai(status = 200)]
    Ok(Json<RespData>),
}
