mod resp;
pub use resp::{Resp, RespData};

mod req;
mod service;
pub use req::{LoginReq, Pagination};
mod auth;

pub use auth::{ApiAuth, User, SERVER_KEY};

pub use service::ApiService;
