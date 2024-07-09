use anyhow::{anyhow, Result};
use ethers::prelude::transaction::eip712::Eip712;
use ethers::types::U256;
use ethers::types::{Address, Signature};
use ethers::utils::hex;
use iri_string::types::UriString;
use poem::http::uri::Authority;
use poem_openapi::types::{ToJSON, Type};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use siwe::{Message, TimeStamp, Version};
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub static DEFAULT_EXPIRY_MIN: i64 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct LoginReq {
    /// sign.v
    pub v: u64,
    /// sign.r
    pub r: String,
    /// sign.s
    pub s: String,

    pub domain: String,
    pub address: String,
    pub statement: Option<String>,
    /// An RFC 3986 URI referring to the resource that is the subject of the signing (as in the subject of a claim).
    pub uri: String,
    /// The current version of the message, which MUST be 1 for this specification.
    pub version: String,
    /// The EIP-155 Chain ID to which the session is bound, and the network where Contract Accounts MUST be resolved.
    pub chain_id: u64,
    /// A randomized token typically chosen by the relying party and used to prevent replay attacks, at least 8 alphanumeric characters.
    pub nonce: String,
    /// The ISO 8601 datetime string of the current time.
    pub issued_at: String,
    /// The ISO 8601 datetime string that, if present, indicates when the signed authentication message is no longer valid.
    pub expiration_time: Option<String>,
    /// The ISO 8601 datetime string that, if present, indicates when the signed authentication message will become valid.
    pub not_before: Option<String>,
    /// An system-specific identifier that may be used to uniquely refer to the sign-in request.
    pub request_id: Option<String>,
    /// A list of information or references to information the user wishes to have resolved as part of authentication by the relying party. They are expressed as RFC 3986 URIs separated by "\n- " where \n is the byte 0x0a.
    pub resources: Vec<String>,
}

pub struct LoginReqParam {
    pub signature: Signature,
    pub msg: Message,
    pub now: OffsetDateTime,
    pub expiry_time: OffsetDateTime,
}

impl LoginReq {
    pub fn to_param(&self) -> Result<LoginReqParam> {
        let r = U256::from_str(&self.r)?;
        let s = U256::from_str(&self.s)?;

        let mut resources = vec![];

        for v in &self.resources {
            let u = UriString::from_str(v)?;
            resources.push(u);
        }

        let now = OffsetDateTime::now_utc();

        let (expiration_time, expiry_time) = if let Some(v) = &self.expiration_time {
            let expiry_time = OffsetDateTime::parse(v, &Rfc3339)?;
            (Some(TimeStamp::from_str(v)?), expiry_time)
        } else {
            let expiry_time = now + Duration::minutes(DEFAULT_EXPIRY_MIN);
            (None, expiry_time)
        };

        let not_before = if let Some(v) = &self.not_before {
            Some(TimeStamp::from_str(v)?)
        } else {
            None
        };

        let msg = Message {
            domain: Authority::from_str(&self.domain)?,
            address: Address::from_str(&self.address)?.0,
            statement: self.statement.clone(),
            uri: UriString::from_str(&self.uri)?,
            version: Version::from_str(&self.version)?,
            chain_id: self.chain_id,
            nonce: self.nonce.clone(),
            issued_at: TimeStamp::from_str(&self.issued_at)?,
            expiration_time,
            not_before,
            request_id: self.request_id.clone(),
            resources,
        };

        Ok(LoginReqParam {
            msg,
            now,
            signature: Signature { r, s, v: self.v },
            expiry_time,
        })
    }
}

impl LoginReqParam {
    pub fn check(&self, block_num: u64, chain_id: u64, domain: &Authority) -> Result<()> {
        // check

        // check nonce(block_number)
        // TODO: Is some different authentication strategy needed?
        {
            if self.msg.nonce.is_empty() {
                return Err(anyhow!("eip4361 msg nonce is nil"));
            }
        }

        // check domain
        {
            if domain != &self.msg.domain {
                return Err(anyhow!("verify domain fail"));
            }
        }

        // check chain_id
        {
            if chain_id != self.msg.chain_id {
                return Err(anyhow!("verify chain_id fail"));
            }
        }

        // check issued-at

        // check address
        {
            let sig: [u8; 65] = self.signature.into();

            self.msg.verify_eip191(&sig)?;
        }

        Ok(())
    }
}
