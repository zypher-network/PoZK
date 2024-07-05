use std::str::FromStr;
use ethers::abi::{encode, encode_packed, Hash, Token, Uint};
use ethers::core::k256::{PublicKey};
use ethers::types::U256;
use ethers::prelude::transaction::eip712::{Eip712, TypedData};
use ethers::utils::{hex, keccak256};
use once_cell::sync::{Lazy, OnceCell};
use serde_json::Value;
use anyhow::{anyhow, Result};
use ethers::types::{Address, Signature};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use crate::poem::service::EIP712_DOMAIN_NAME;
use crate::poem::utils::set_domain;

pub static LOGIN_MSG_TYPE: &str = "Message(bytes publicKey,uint256 randomNum)";

pub static LOGIN_MESSAGE_TYPE_TEMPLATE: Lazy<TypedData> = Lazy::new(|| {
    let json = serde_json::json!(
        {
            "types": {
                "EIP712Domain": [
                    {
                        "name": "name",
                        "type": "string"
                    },
                    {
                        "name": "version",
                        "type": "string"
                    },
                    {
                        "name": "chainId",
                        "type": "uint256"
                    },
                    {
                        "name": "verifyingContract",
                        "type": "address"
                    }
                ],
                "Message": [
                    {
                        "name": "nonce",
                        "type": "uint256"
                    },
                    {
                        "name": "address",
                        "type": "address"
                    }
                ],
            },
            "domain": {},
            "primaryType": "Message",
            "message": {}
        }
    );
    let typed_data: TypedData = serde_json::from_value(json).unwrap();
    typed_data
});

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct LoginReq {
    /// 地址
    pub address: String,

    /// sign.v
    pub v: u64,
    /// sign.r
    pub r: String,
    /// sign.s
    pub s: String,
}

pub struct LoginReqParam {
    pub address: Address,
    pub signature: Signature,
}

impl LoginReq {
    pub fn to_param(&self) -> Result<LoginReqParam> {

        let address = Address::from_str(&self.address)?;
        let r = U256::from_str(&self.r)?;
        let s = U256::from_str(&self.s)?;

        Ok(LoginReqParam {
            address,
            signature: Signature{
                r,
                s,
                v: self.v,
            }
        })

    }
}

impl LoginReqParam {
    pub fn login_hash_eip712(
        &self,
        nonce: u64,
        chain_id: u64,
    ) -> Result<Hash> {
        let mut login_message_template = LOGIN_MESSAGE_TYPE_TEMPLATE.clone();

        set_domain(&mut login_message_template, chain_id);

        login_message_template.message.insert(
            "nonce".to_string(),
            Value::String(nonce.to_string())
        );

        login_message_template.message.insert(
            "address".to_string(),
            Value::String(format!("{:?}",self.address))
        );

        let hash = login_message_template.encode_eip712()?;

        Ok(Hash::from(hash))
    }

    pub fn check(&self, nonce: u64, chain_id: u64) -> Result<()> {
        let eip712_hash = self.login_hash_eip712(nonce, chain_id)?;

        let address = self.signature.recover(eip712_hash)?;

        return if self.address.eq(&address) {
            Ok(())
        } else {
            log::error!("server calculate address: {eip712_hash:?}, post address: {:?}", self.address);
            Err(anyhow!("server calculate address != post address"))
        }
    }
}





