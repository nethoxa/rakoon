use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{builder::Builder, types::Backend};
use rand::{SeedableRng, rngs::StdRng};

pub struct RandomTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub tx_sent: u64,
    pub provider: Backend,
}

impl Builder for RandomTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }
}

impl RandomTransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url.parse().unwrap());

        Self { sk, seed, tx_sent: 0, provider }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);

        loop {
            let tx = self.create_random_transaction(&mut random, sender).await;
            let _ = self.provider.send_transaction(tx).await;
            self.tx_sent += 1;
        }
    }

    pub async fn create_random_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> TransactionRequest {
        let to = self.to(random);

        let gas_price = self.gas_price(random).await;

        let max_fee_per_gas = self.max_fee_per_gas(random);

        let max_priority_fee_per_gas = self.max_priority_fee_per_gas(random).await;

        let max_fee_per_blob_gas = self.max_fee_per_blob_gas(random).await;

        let gas = self.gas(random);

        let value = self.value(random, sender).await;

        let input = self.input(random);

        let nonce = self.nonce(random, sender).await;

        let chain_id = self.chain_id(random).await;

        let access_list = self.access_list(random);

        let transaction_type = self.transaction_type(random);

        let blob_versioned_hashes = self.blob_versioned_hashes(random);

        let sidecar = self.sidecar(random);

        let authorization_list = self.authorization_list(random);

        TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: Some(max_fee_per_gas),
            max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
            max_fee_per_blob_gas: Some(max_fee_per_blob_gas),
            gas: Some(gas),
            value: Some(value),
            input,
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: Some(access_list),
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: Some(blob_versioned_hashes),
            sidecar: Some(sidecar),
            authorization_list: Some(authorization_list),
        }
    }
}

#[tokio::test]
async fn test_random_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = RandomTransactionRunner::new(
        "http://localhost:8545".to_string(),
        SigningKey::from_slice(
            &alloy::hex::decode(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_random_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
