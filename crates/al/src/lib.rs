use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{builder::Builder, types::Backend};
use rand::{SeedableRng, rngs::StdRng};

pub struct ALTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub tx_sent: u64,
    pub provider: Backend,
}

impl Builder for ALTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }
}

impl ALTransactionRunner {
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
            let tx = self.create_access_list_transaction(&mut random, sender).await;
            let _ = self.provider.send_transaction(tx).await;
            self.tx_sent += 1;
        }
    }

    pub async fn create_access_list_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> TransactionRequest {
        let to = self.to(random);

        let gas_price = self.gas_price(random).await;

        let gas_limit = self.gas(random);

        let value = self.value(random, sender).await;

        let input = self.input(random);

        let nonce = self.nonce(random, sender).await;

        let chain_id = self.chain_id(random).await;

        let access_list = self.access_list(random);

        // EIP-2930 transaction type
        let transaction_type = 1;

        TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            max_fee_per_blob_gas: None,
            gas: Some(gas_limit),
            value: Some(value),
            input,
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: Some(access_list),
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: None,
            sidecar: None,
            authorization_list: None,
        }
    }
}

#[tokio::test]
async fn test_access_list_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = ALTransactionRunner::new(
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
    let tx = runner.create_access_list_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
