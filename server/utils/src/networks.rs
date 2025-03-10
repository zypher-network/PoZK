use anyhow::{anyhow, Result};
use ethers::{
    abi::AbiEncode,
    prelude::*,
    types::transaction::{
        eip2718::TypedTransaction,
        eip712::{EIP712Domain, Eip712DomainType, TypedData},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::sync::Arc;
use tracing::error;

const GAS_PRICE: u64 = 50;

// Vesting contract with abi
abigen!(Token, "public/ABI/Token.json");

// Vesting contract with abi
abigen!(Vesting, "public/ABI/Vesting.json");

// Epoch contract with abi
abigen!(Epoch, "public/ABI/Epoch.json");

// Prover contract with abi
abigen!(Prover, "public/ABI/Prover.json");

// Stake contract with abi
abigen!(Stake, "public/ABI/Stake.json");

// Task contract with abi
abigen!(Task, "public/ABI/Task.json");

// Reward contract with abi
abigen!(Reward, "public/ABI/Reward.json");

// Controller contract with abi
abigen!(Controller, "public/ABI/Controller.json");

// Zytron standard AA wallet for zero gas
abigen!(AAWallet, "public/others/Wallet.json");

/// Prover type
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ProverType {
    ZK,
    ZK_VM,
    Z4,
    AI_MODEL,
    AI_AGENT,
}

impl ProverType {
    pub fn to_byte(&self) -> u8 {
        match self {
            ProverType::ZK => 0u8,
            ProverType::ZK_VM => 1u8,
            ProverType::Z4 => 2u8,
            ProverType::AI_MODEL => 3u8,
            ProverType::AI_AGENT => 4u8,
        }
    }

    pub fn from_byte(b: u8) -> Self {
        match b {
            1u8 => ProverType::ZK_VM,
            2u8 => ProverType::Z4,
            3u8 => ProverType::AI_MODEL,
            4u8 => ProverType::AI_AGENT,
            _ => ProverType::ZK,
        }
    }

    pub fn check_url(&self) -> bool {
        match self {
            ProverType::ZK | ProverType::ZK_VM => false,
            _ => true,
        }
    }

    pub fn is_zkvm(&self) -> bool {
        match self {
            ProverType::ZK_VM => true,
            _ => false,
        }
    }
}

const NETWORKS_ADDRESS: &str = include_str!("../public/networks.json");

pub fn contract_address(network: &str, name: &str) -> Result<(Address, u64)> {
    let addresses: Value =
        serde_json::from_str(NETWORKS_ADDRESS).map_err(|_| anyhow!("networks.json is invalid"))?;

    let address: Address = addresses[network][name]["address"]
        .as_str()
        .ok_or(anyhow!("contract address is invalid 1"))?
        .parse()
        .map_err(|_| anyhow!("contract address is invalid 2"))?;

    let start: i64 = addresses[network][name]["startBlock"]
        .as_i64()
        .ok_or(anyhow!("contract block is invalid"))?;

    Ok((address, start as u64))
}

pub fn pozk_metrics_url(network: &str) -> Result<String> {
    match network {
        "localhost" | "zytrontestnet" | "basesepolia" => {
            Ok("https://pozk-proxy.zypher.dev".to_owned())
        }
        "zytron" | "base" => Ok("https://pozk-proxy.zypher.network".to_owned()),
        _ => Err(anyhow!("Invalid network")),
    }
}

pub fn pozk_rpc_url(network: &str) -> Result<String> {
    match network {
        "localhost" => Ok("http://localhost:8545".to_owned()),
        "zytrontestnet" => Ok("https://rpc-testnet.zypher.network".to_owned()),
        "zytron" => Ok("https://rpc.zypher.network".to_owned()),
        "basesepolia" => Ok("https://sepolia.base.org".to_owned()),
        "base" => Ok("https://mainnet.base.org".to_owned()),
        _ => Err(anyhow!("Invalid network")),
    }
}

pub fn pozk_zero_gas_url(network: &str) -> Result<String> {
    match network {
        "zytrontestnet" => Ok("https://gas-testnet.zypher.network".to_owned()),
        "zytron" => Ok("https://gas.zypher.network".to_owned()),
        "basesepolia" => Ok("https://gas-basesepolia.zypher.dev".to_owned()),
        "base" => Ok("https://gas-base.zypher.dev".to_owned()),
        _ => Err(anyhow!("Invalid network")),
    }
}

pub fn pozk_gas_price(network: &str) -> Option<U256> {
    match network {
        "zytrontestnet" | "zytron" => Some(U256::from(GAS_PRICE)),
        "basesepolia" | "base" => None,
        _ => None,
    }
}

pub type DefaultProvider = Provider<Http>;

pub fn new_providers(rpcs: &[String]) -> Vec<Arc<DefaultProvider>> {
    let mut providers = vec![];
    for rpc in rpcs {
        if let Ok(p) = Provider::<Http>::try_from(rpc) {
            providers.push(Arc::new(p))
        }
    }
    providers
}

pub type DefaultSigner = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

pub async fn new_signer(
    provider: Arc<DefaultProvider>,
    wallet: LocalWallet,
) -> Result<Arc<DefaultSigner>> {
    let signer = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;
    Ok(Arc::new(signer))
}

pub async fn check_zero_gas(uri: &str, controller: Address) -> Result<()> {
    let res = reqwest::get(format!("{}/balanceof/{:?}", uri, controller))
        .await?
        .json::<Value>()
        .await?;
    if let Some(amount) = res.pointer("/amount") {
        if amount != "0x0" {
            return Ok(());
        }
    }

    Err(anyhow!("No permission"))
}

pub async fn create_zero_gas(uri: &str, controller: Address) -> Result<Address> {
    let client = reqwest::Client::new();
    let data = json!({
        "owner": format!("{:?}", controller)
    });
    let res = client
        .post(format!("{}/create", uri))
        .json(&data)
        .send()
        .await?
        .json::<Value>()
        .await?;
    if let Some(aa) = res.pointer("/wallet") {
        Ok(aa.as_str().unwrap_or("").parse::<Address>()?)
    } else {
        Err(anyhow!("Invalid response"))
    }
}

pub async fn zero_gas<S: Signer>(
    uri: &str,
    tx: TypedTransaction,
    chain: u64,
    aa: Address,
    nonce: u64,
    wallet: &S,
) -> Result<Option<String>> {
    let owner = format!("{:?}", wallet.address());
    let from = format!("{:?}", aa);

    let to = format!("{:?}", tx.to_addr().unwrap());
    let value = tx.value().unwrap_or(&U256::zero()).encode_hex();
    let data = format!("0x{}", hex::encode(tx.data().unwrap().to_vec()));

    let tdata = generate_eip712_data(chain, nonce, &from, &to, &value, &data);
    let sign = wallet
        .sign_typed_data(&tdata)
        .await
        .map_err(|_| anyhow!("Invalid typed data"))?;
    let mut r_bytes = [0u8; 32];
    sign.r.to_big_endian(&mut r_bytes);
    let mut s_bytes = [0u8; 32];
    sign.s.to_big_endian(&mut s_bytes);

    let v = sign.v;
    let r = format!("0x{}", hex::encode(&r_bytes));
    let s = format!("0x{}", hex::encode(&s_bytes));

    let client = reqwest::Client::new();
    let data = json!({
        "wallet": from,
        "to": to,
        "data": data,
        "value": value,
        "v": v,
        "r": r,
        "s": s,
        "owner": owner,
    });
    let res = client
        .post(format!("{}/functioncall", uri))
        .json(&data)
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(hash) = res.pointer("/tx_hash") {
        return Ok(Some(hash.as_str().unwrap_or("").to_owned()));
    }

    if let Some(err) = res.pointer("/error") {
        error!("[Utils] 0 gas chain error: {}", err);
        Ok(None)
    } else {
        Err(anyhow!("Something wrong with 0 gas service"))
    }
}

// EIP712Domain(string name,uint256 chainId)
// Message(string tip,uint256 nonce,address from,address to,uint256 value,bytes data)
const DOMAIN: &str = "Zytron";
const FUNCTION_CALL_TIP: &str =
    "You are agreeing to this single transaction to be executed on the chain.";

#[inline]
fn eip712_type(name: &str, t: &str) -> Eip712DomainType {
    Eip712DomainType {
        name: name.to_owned(),
        r#type: t.to_owned(),
    }
}

fn generate_eip712_data(
    chain: u64,
    nonce: u64,
    from: &str,
    to: &str,
    value: &str,
    data: &str,
) -> TypedData {
    let mut types = BTreeMap::new();
    types.insert(
        "EIP712Domain".to_owned(),
        vec![
            eip712_type("name", "string"),
            eip712_type("chainId", "uint256"),
        ],
    );
    types.insert(
        "Message".to_owned(),
        vec![
            eip712_type("tip", "string"),
            eip712_type("nonce", "uint256"),
            eip712_type("from", "address"),
            eip712_type("to", "address"),
            eip712_type("value", "uint256"),
            eip712_type("data", "bytes"),
        ],
    );

    let mut message = BTreeMap::new();
    message.insert("tip".to_owned(), FUNCTION_CALL_TIP.into());
    message.insert("nonce".to_owned(), nonce.into());
    message.insert("from".to_owned(), from.into());
    message.insert("to".to_owned(), to.into());
    message.insert("value".to_owned(), value.into());
    message.insert("data".to_owned(), data.into());

    TypedData {
        types,
        message,
        domain: EIP712Domain {
            name: Some(DOMAIN.to_owned()),
            chain_id: Some(chain.into()),
            ..Default::default()
        },
        primary_type: "Message".to_owned(),
    }
}

const PROXY_LIST_ACCOUNTS: [H160; 1] = [H160([
    94, 245, 28, 159, 68, 157, 183, 190, 47, 12, 99, 108, 108, 19, 126, 101, 184, 185, 107, 155,
])];

pub fn check_task_proxy_list(signer: &Address) -> bool {
    PROXY_LIST_ACCOUNTS.contains(signer)
}

#[tokio::test]
async fn test_zero_gas() {
    let uri = "https://gas.zypher.network";
    let account: Address = "0x5Ef51c9f449DB7Be2f0c636C6C137e65B8B96B9B"
        .parse()
        .unwrap();
    let wallet: Address = "0x4e3111334ba387ddf000966cde24db35245fdc59"
        .parse()
        .unwrap();
    assert!(check_zero_gas(uri, account).await.is_ok());
    assert_eq!(create_zero_gas(uri, account).await.unwrap(), wallet);

    let account2: Address = "0x1Ef51c9f449DB7Be2f0c636C6C137e65B8B96B9B"
        .parse()
        .unwrap();
    assert!(check_zero_gas(uri, account2).await.is_err());
}
