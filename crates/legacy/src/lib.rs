use alloy::signers::k256::ecdsa::SigningKey;
use common::errors::Error;
use tokio_util::sync::CancellationToken;
pub struct LegacyTransactionRunner {
    rpc_url: String,
    sk: SigningKey,
    seed: u64,
}

impl LegacyTransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        Self { rpc_url, sk, seed }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<(), Error> {
        Ok(())
    }
}
