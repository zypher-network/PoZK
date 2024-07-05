use ethers::prelude::transaction::eip712::TypedData;
use crate::poem::service::EIP712_DOMAIN_NAME;

pub fn set_domain(td: &mut TypedData, chain_id: u64) {
    td.domain.name = Some(EIP712_DOMAIN_NAME.to_string());
    td.domain.chain_id = Some(ethers::prelude::U256::from(chain_id));
}