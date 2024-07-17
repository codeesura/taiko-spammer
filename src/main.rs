use alloy::primitives::U256;
use eyre::Result;
use transactions::create_transaction::{create_transaction, TransactionParams};
mod transactions;
use std::str::FromStr;
mod utils;

use crate::utils::{config::Config, constants::DEPOSIT_ADDRESS, provider};
use alloy::{
    primitives::{Address, Bytes},
    sol,
    sol_types::SolCall,
};
use std::sync::Arc;

const TRANSACTION_BATCH_SIZE: usize = 10;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    function deposit() external;
    function withdraw(uint256 wad) external;
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let provider = Arc::new(provider::setup().await?);

    loop {
        let gas_price = provider.get_gas_price().await?;
        let mut nonce = provider
            .get_transaction_count(config.spammer_address)
            .await?;
        let value = U256::from_str("100000000000000")?;

        let mut raw_transactions = Vec::with_capacity(TRANSACTION_BATCH_SIZE);

        for i in 0..TRANSACTION_BATCH_SIZE {
            let params = if i % 2 == 0 {
                // Deposit transaction
                let input = Bytes::from(depositCall {}.abi_encode());
                TransactionParams {
                    signer: config.spammer_private_key.clone(),
                    to: Address::from_str(DEPOSIT_ADDRESS)?,
                    input: Some(input),
                    value: Some(value),
                    gas_limit: 50_000,
                    nonce,
                    gas_price,
                }
            } else {
                // Withdraw transaction
                let input = Bytes::from(withdrawCall { wad: value }.abi_encode());
                TransactionParams {
                    signer: config.spammer_private_key.clone(),
                    to: Address::from_str(DEPOSIT_ADDRESS)?,
                    input: Some(input),
                    value: None,
                    gas_limit: 50_000,
                    nonce,
                    gas_price,
                }
            };

            let raw_tx = create_transaction(params).await?;
            raw_transactions.push(raw_tx);
            nonce += 1;
        }

        let send_futures = raw_transactions.into_iter().enumerate().map(|(i, raw_tx)| {
            let provider_clone = Arc::clone(&provider);
            tokio::spawn(async move {
                match provider_clone.send_tx_envelope(raw_tx).await {
                    Ok(receipt) => {
                        match receipt.get_receipt().await {
                            Ok(tx_receipt) => {
                                println!("Transaction {} sent with hash: {:?}", i, tx_receipt)
                            }
                            Err(e) => {
                                println!("Failed to get receipt for transaction {}: {:?}", i, e)
                            }
                        }
                        Ok(())
                    }
                    Err(e) => {
                        println!("Failed to send transaction {}: {:?}", i, e);
                        Err(())
                    }
                }
            })
        });

        futures::future::join_all(send_futures).await;

        println!("All transactions in batch sent");
    }
}
