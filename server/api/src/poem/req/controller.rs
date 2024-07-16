use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::Address;
use ethers::utils::hex::decode;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ControllerAddReq {
    /// ECDSA/ secp256k1 signing key
    pub signing_key: String,
}

pub struct ControllerAddParam {
    pub signing_key: SigningKey,
}

impl ControllerAddReq {
    pub fn to_param(&self) -> Result<ControllerAddParam> {
        let signing_key = {
            let bytes = decode(&self.signing_key)?;
            SigningKey::from_slice(&bytes)?
        };

        Ok(ControllerAddParam { signing_key })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ControllerSetReq {
    pub address: String,
}

// pub struct ControllerSetParam {
//     pub address: Address,
// }
//
// impl ControllerSetReq {
//     pub fn to_param(&self) -> Result<ControllerSetParam> {
//         let address = Address::from_str(&self.address)?;
//         Ok(ControllerSetParam { address })
//     }
// }
