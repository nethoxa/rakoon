use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{AccessList, AccessListItem, TransactionInput, TransactionRequest},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{
    Backend,
    constants::{MAX_ACCESS_LIST_LENGTH, MAX_ACCESSED_KEYS_LENGTH, MAX_INPUT_LENGTH},
    errors::Error,
    transaction_builder::TransactionBuilder,
};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub struct EIP1559TransactionRunner {
    builder: TransactionBuilder,
}

impl EIP1559TransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        Self { 
            builder: TransactionBuilder::new(rpc_url, sk, seed)
        }
    }

    pub async fn run(&mut self, token: CancellationToken) -> Result<(), Error> {
        let mut random = StdRng::seed_from_u64(self.builder.get_seed());
        let sender = Address::from_private_key(self.builder.get_signing_key());
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(self.builder.get_signing_key().clone().into())
            .connect_http(self.builder.get_rpc_url().parse().unwrap());

        'outer: loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break 'outer;
                }
                _ = async {
                    let tx = self.create_eip1559_transaction(&mut random, &provider, sender).await;
                    let _ = provider.send_transaction(tx).await.unwrap();
                    self.builder.increment_tx_sent();
                    tokio::time::sleep(Duration::from_millis(10)).await;
                } => {}
            }
        }

        Ok(())
    }

    pub async fn create_eip1559_transaction(
        &self,
        random: &mut StdRng,
        provider: &Backend,
        sender: Address,
    ) -> TransactionRequest {
        let to = if random_bool(0.5) {
            if random_bool(0.5) { TxKind::Create } else { TxKind::Call(self.builder.random_to(random)) }
        } else {
            if random_bool(0.5) { TxKind::Create } else { TxKind::Call(Address::ZERO) }
        };

        let max_fee_per_gas = if random_bool(0.5) {
            self.builder.random_gas_price(random)
        } else {
            provider.get_gas_price().await.unwrap()
        };

        let max_priority_fee_per_gas = if random_bool(0.5) {
            self.builder.random_gas_price(random)
        } else {
            provider.get_max_priority_fee_per_gas().await.unwrap()
        };

        let gas_limit = if random_bool(0.5) {
            self.builder.random_gas(random)
        } else {
            0 // TODO
        };

        let value = if random_bool(0.5) {
            self.builder.random_u256(random)
        } else {
            provider.get_account(sender).await.unwrap().balance / U256::from(100_000_000)
        };

        let input = if random_bool(0.5) {
            self.builder.random_input(random, MAX_INPUT_LENGTH)
        } else {
            TransactionInput::from(vec![])
        };

        let nonce = if random_bool(0.5) {
            self.builder.random_nonce(random)
        } else {
            provider.get_account(sender).await.unwrap().nonce
        };

        let chain_id = if random_bool(0.5) {
            self.builder.random_chain_id(random)
        } else {
            provider.get_chain_id().await.unwrap()
        };

        let access_list =
            if random_bool(0.5) { self.builder.random_access_list(random, MAX_ACCESS_LIST_LENGTH) } else { AccessList::default() };

        // EIP-1559 transaction type
        let transaction_type = 2;

        TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: None,
            max_fee_per_gas: Some(max_fee_per_gas),
            max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
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
async fn test_eip1559_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let provider = ProviderBuilder::new()
        .wallet::<PrivateKeySigner>(
            SigningKey::from_slice(
                &hex::decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                    .unwrap(),
            )
            .unwrap()
            .into(),
        )
        .connect_http("http://localhost:8545".parse().unwrap());
    let runner = EIP1559TransactionRunner::new(
        "http://localhost:8545".to_string(),
        SigningKey::from_slice(
            &hex::decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_eip1559_transaction(&mut rng, &provider, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
