use ethers::prelude::rand::thread_rng;
use ethers::prelude::{LocalWallet, Signer};
use ethers::types::Address;
use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::SecurityScheme;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

type ServerKey = Hmac<Sha256>;
pub static SERVER_KEY: Lazy<ServerKey> = Lazy::new(|| {
    let wallet = LocalWallet::new(&mut thread_rng());
    let address = wallet.address();
    Hmac::<Sha256>::new_from_slice(address.as_bytes()).unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub address: Address,
    pub create_time: i64,
    pub expiry_time: i64,
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker"
)]
pub struct ApiAuth(pub User);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let server_key = req.data::<ServerKey>().unwrap();
    let Some(user) = VerifyWithKey::<User>::verify_with_key(api_key.key.as_str(), server_key).ok()
    else {
        return None;
    };

    // check expiry
    {
        let now = OffsetDateTime::now_utc();
        let expiry = OffsetDateTime::from_unix_timestamp(user.expiry_time);
        if let Err(e) = expiry {
            log::error!("check user expiry err: {e:?}");
            return None;
        }

        let expiry = expiry.unwrap();

        if expiry < now {
            log::debug!("token: {}, Timed out: {:?}", api_key.key.as_str(), expiry);
            return None;
        }
    }

    Some(user)
}
