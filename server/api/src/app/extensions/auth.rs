use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
    Extension,
};
use chrono::prelude::*;
use ethers::types::Address;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use siwe::{Message, VerificationOpts};

use crate::app::{AppContext, Error, Result};

pub struct Auth;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// issue timestamp
    pub iat: i64,
    /// token expiration
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Erc4361Payload {
    message: String,
    signature: String,
}

impl Erc4361Payload {
    pub async fn verify(
        &self,
        secret: &[u8],
        domains: &[String],
        miner: &Address,
    ) -> Result<String> {
        let message: Message = self.message.parse().map_err(|_| Error::Auth)?;
        let signature =
            hex::decode(self.signature.trim_start_matches("0x")).map_err(|_| Error::Auth)?;

        // check domain
        let host = message.domain.host().to_owned();
        if !domains.is_empty() && !domains.contains(&host) {
            return Err(Error::Auth);
        }

        // check account
        if message.address != &miner[..] {
            return Err(Error::Auth);
        }

        // check issued-at
        let now = Utc::now().timestamp();
        let expiration = now + (7 * 86400);
        let iat = message.issued_at.as_ref().unix_timestamp();
        if (now - iat).abs() > 300 {
            // 5min
            return Err(Error::Auth);
        }

        if message
            .verify(&signature, &VerificationOpts::default())
            .await
            .is_ok()
        {
            let header = Header::new(Algorithm::HS512);
            let claims = Claims {
                iat,
                exp: expiration,
            };

            encode(&header, &claims, &EncodingKey::from_secret(secret)).map_err(|_| Error::Auth)
        } else {
            Err(Error::Auth)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        req: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // inject user to context
        let Extension(context): Extension<AppContext> = Extension::from_request_parts(req, state)
            .await
            .map_err(|_| Error::Internal(2056))?;

        // Get authorisation header
        let authorisation = req
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Auth)?
            .to_str()
            .map_err(|_| Error::Auth)?;

        // Check that is bearer and jwt
        let split = authorisation.split_once(' ');
        let jwt = match split {
            Some((name, contents)) if name == "Bearer" => Ok(contents),
            _ => Err(Error::Auth),
        }?;

        let decoded = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(&context.secret),
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|_| Error::Auth)?;

        if decoded.claims.exp < Utc::now().timestamp() {
            return Err(Error::Auth);
        }

        Ok(Auth)
    }
}
