use crate::utils::constants::CHAIN_ID;
use alloy::{
    consensus::TxEnvelope,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

pub struct TransactionParams {
    pub signer: PrivateKeySigner,
    pub to: Address,
    pub input: Option<Bytes>,
    pub value: Option<U256>,
    pub gas_limit: u128,
    pub nonce: u64,
    pub gas_price: u128,
}

pub async fn create_transaction(params: TransactionParams) -> Result<TxEnvelope> {
    let mut tx = TransactionRequest::default()
        .with_to(params.to)
        .with_value(params.value.unwrap_or_default())
        .with_gas_limit(params.gas_limit)
        .with_nonce(params.nonce)
        .with_gas_price(params.gas_price)
        .with_max_fee_per_gas(params.gas_price)
        .with_max_priority_fee_per_gas(params.gas_price)
        .with_chain_id(CHAIN_ID);

    if let Some(input) = params.input {
        tx = tx.with_input(input);
    }

    Ok(tx.build(&EthereumWallet::from(params.signer)).await?)
}
