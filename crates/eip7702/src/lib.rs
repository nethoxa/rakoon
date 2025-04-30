use common::errors::Error;
use tokio_util::sync::CancellationToken;
use alloy::signers::k256::ecdsa::SigningKey;

pub struct EIP7702TransactionRunner {
    rpc_url: String,
    sk: SigningKey,
    seed: u64,
}

impl EIP7702TransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        Self { rpc_url, sk, seed }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<(), Error> {
        Ok(())
    }
}
