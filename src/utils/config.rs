use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub spammer_private_key: PrivateKeySigner,
    pub spammer_address: Address,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let spammer_private_key: PrivateKeySigner = env::var("PRIVATE_KEY_SPAMMER")
            .expect("PRIVATE_KEY_SPAMMER must be set")
            .parse()
            .expect("should parse private key");

        let spammer_address = spammer_private_key.address();

        Config {
            rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
            spammer_private_key,
            spammer_address,
        }
    }
}
