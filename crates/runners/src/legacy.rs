use crate::builder::Builder;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use common::types::Backend;
use rand::{SeedableRng, rngs::StdRng};
pub struct LegacyTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub tx_sent: u64,
    pub provider: Backend,
}

impl Builder for LegacyTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }
}

impl LegacyTransactionRunner {
    pub fn new(rpc_url: Url, sk: SigningKey, seed: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url);
        Self { sk, seed, tx_sent: 0, provider }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);

        loop {
            let tx = self.create_legacy_transaction(&mut random, sender).await;
            let _ = self.provider.send_transaction(tx).await;
            self.tx_sent += 1;
        }
    }

    pub async fn create_legacy_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> TransactionRequest {
        let to = self.to(random);

        let gas_price = self.gas_price(random).await;

        let gas = self.gas(random);

        let value = self.value(random, sender).await;

        let input = self.input(random);

        let nonce = self.nonce(random, sender).await;

        let chain_id = self.chain_id(random).await;

        let transaction_type = 0;

        TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            max_fee_per_blob_gas: None,
            gas: Some(gas),
            value: Some(value),
            input,
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: None,
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: None,
            sidecar: None,
            authorization_list: None,
        }
    }
}

#[tokio::test]
async fn test_legacy_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = LegacyTransactionRunner::new(
        "http://localhost:8545".parse::<Url>().unwrap(),
        SigningKey::from_slice(
            &alloy::hex::decode(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_legacy_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
