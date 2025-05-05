use crate::builder::Builder;
use alloy::{
    consensus::TxEip7702,
    primitives::{Address, TxHash},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use alloy_rlp::Encodable;
use common::types::Backend;
use mutator::Mutator;
use rand::{SeedableRng, random_bool, rngs::StdRng};

pub struct EIP7702TransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub provider: Backend,
    pub current_tx: Vec<u8>,
    pub mutator: Mutator,
}

impl Builder for EIP7702TransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }
}

impl EIP7702TransactionRunner {
    pub fn new(rpc_url: Url, sk: SigningKey, seed: u64, max_operations_per_mutation: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url);

        let mutator = Mutator::new(max_operations_per_mutation, seed);

        Self { sk, seed, current_tx: vec![], provider, mutator }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);

        loop {
            // 10% chance to re-generate the transaction
            if random_bool(0.1) || self.current_tx.is_empty() {
                let (request, tx) = self.create_eip7702_transaction(&mut random, sender).await;
                tx.encode(&mut self.current_tx);

                let _ = self.provider.send_transaction_unsafe(request).await;
            } else {
                self.mutator.mutate(&mut self.current_tx);
                let _: Result<TxHash, _> = self
                    .provider
                    .client()
                    .request("eth_sendRawTransaction", &self.current_tx)
                    .await;
            }
        }
    }

    pub async fn create_eip7702_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> (TransactionRequest, TxEip7702) {
        // EIP-7702 transaction type
        let transaction_type = 4;

        let to = self.to(random);
        let max_fee_per_gas = self.max_fee_per_gas(random);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas(random).await;
        let gas_limit = self.gas(random);
        let value = self.value(random, sender).await;
        let input = self.input(random);
        let nonce = self.nonce(random, sender).await;
        let chain_id = self.chain_id(random).await;
        let access_list = self.access_list(random);
        let authorization_list = self.authorization_list(random);

        let request = TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: None,
            max_fee_per_gas: Some(max_fee_per_gas),
            max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
            max_fee_per_blob_gas: None,
            gas: Some(gas_limit),
            value: Some(value),
            input: input.clone(),
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: Some(access_list.clone()),
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: None,
            sidecar: None,
            authorization_list: Some(authorization_list.clone()),
        };

        let tx = TxEip7702 {
            to: to.into_to().unwrap_or_else(|| Address::ZERO),
            gas_limit,
            value,
            chain_id,
            nonce,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            access_list,
            authorization_list,
            input: input.into_input().unwrap(),
        };

        (request, tx)
    }
}

#[tokio::test]
async fn test_eip7702_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = EIP7702TransactionRunner::new(
        "http://localhost:8545".parse::<Url>().unwrap(),
        SigningKey::from_slice(
            &alloy::hex::decode(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .unwrap(),
        )
        .unwrap(),
        1,
        1000,
    );
    let tx = runner.create_eip7702_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
