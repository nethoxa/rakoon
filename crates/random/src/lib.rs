use alloy::{
    consensus::BlobTransactionSidecar,
    eips::{eip4844::BYTES_PER_BLOB, eip7702::SignedAuthorization},
    hex,
    primitives::{Address, Bytes, FixedBytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{AccessList, AccessListItem, Authorization, TransactionInput, TransactionRequest},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{
    Backend,
    constants::{
        MAX_ACCESS_LIST_LENGTH, MAX_ACCESSED_KEYS_LENGTH, MAX_AUTHORIZATION_LIST_LENGTH,
        MAX_BLOB_SIDECAR_LENGTH, MAX_BLOB_VERSIONED_HASHES_LENGTH, MAX_INPUT_LENGTH,
        MAX_TRANSACTION_TYPE,
    },
    errors::Error,
    builder::Builder
};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub struct RandomTransactionRunner {
    pub rpc_url: String,
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

        Self { rpc_url, sk, seed, tx_sent: 0, provider }
    }

    pub async fn run(&mut self, token: CancellationToken) -> Result<(), Error> {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);

        'outer: loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break 'outer;
                }
                _ = async {
                    let tx = self.create_random_transaction(&mut random, sender).await;
                    let _ = self.provider.send_transaction(tx).await.unwrap();
                    self.tx_sent += 1;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                } => {}
            }
        }

        Ok(())
    }

    pub async fn create_random_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> TransactionRequest {
        let to = self.to(random);

        let gas_price = self.gas_price(random);

        let max_fee_per_gas = if random_bool(0.5) {
            self.random_max_fee_per_gas(random)
        } else {
            0 // TODO
        };

        let max_priority_fee_per_gas = if random_bool(0.5) {
            self.random_max_priority_fee_per_gas(random)
        } else {
            provider.get_max_priority_fee_per_gas().await.unwrap()
        };

        let max_fee_per_blob_gas =
            if random_bool(0.5) { self.random_max_fee_per_blob_gas(random) } else { 0 };

        let gas = if random_bool(0.5) {
            self.random_gas(random)
        } else {
            0 // TODO
        };

        let value = if random_bool(0.5) {
            self.random_u256(random)
        } else {
            provider.get_account(sender).await.unwrap().balance / U256::from(100_000_000)
        };

        let input = if random_bool(0.5) {
            self.random_input(random)
        } else {
            TransactionInput::from(vec![])
        };

        let nonce = if random_bool(0.5) {
            self.random_nonce(random)
        } else {
            provider.get_account(sender).await.unwrap().nonce
        };

        let chain_id = if random_bool(0.5) {
            self.random_chain_id(random)
        } else {
            provider.get_chain_id().await.unwrap()
        };

        let access_list = if random_bool(0.5) {
            self.random_access_list(random)
        } else {
            AccessList::from(vec![])
        };

        let transaction_type = if random_bool(0.5) {
            self.random_transaction_type(random)
        } else {
            0 // [nethoxa] should we default to this
        };

        let blob_versioned_hashes =
            if random_bool(0.5) { self.random_blob_versioned_hashes(random) } else { vec![] };

        let sidecar = if random_bool(0.5) {
            self.random_sidecar(random)
        } else {
            BlobTransactionSidecar::new(vec![], vec![], vec![])
        };

        let authorization_list =
            if random_bool(0.5) { self.random_authorization_list(random) } else { vec![] };

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
    let runner = RandomTransactionRunner::new(
        "http://localhost:8545".to_string(),
        SigningKey::from_slice(
            &hex::decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_random_transaction(&mut rng, &provider, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
