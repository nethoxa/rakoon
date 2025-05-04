use alloy::{
    consensus::TxLegacy,
    hex,
    primitives::{Address, Bytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionInput, TransactionRequest},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{Backend, constants::MAX_INPUT_LENGTH, errors::Error};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub struct LegacyTransactionRunner {
    rpc_url: String,
    sk: SigningKey,
    seed: u64,
    tx_sent: u64,
}

impl LegacyTransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        Self { rpc_url, sk, seed, tx_sent: 0 }
    }

    pub async fn run(&mut self, token: CancellationToken) -> Result<(), Error> {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(self.sk.clone().into())
            .connect_http(self.rpc_url.parse().unwrap());

        'outer: loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break 'outer;
                }
                _ = async {
                    let tx = self.create_legacy_transaction(&mut random, &provider, sender).await;
                    let _ = provider.send_transaction(tx).await.unwrap();
                    self.tx_sent += 1;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                } => {}
            }
        }

        Ok(())
    }

    pub async fn create_legacy_transaction(
        &self,
        random: &mut StdRng,
        provider: &Backend,
        sender: Address,
    ) -> TransactionRequest {
        let to = if random_bool(0.5) {
            if random_bool(0.5) { TxKind::Create } else { TxKind::Call(self.random_to(random)) }
        } else {
            if random_bool(0.5) { TxKind::Create } else { TxKind::Call(Address::ZERO) }
        };

        let gas_price = if random_bool(0.5) {
            self.random_gas_price(random)
        } else {
            provider.get_gas_price().await.unwrap()
        };

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

    fn random_bytes(&self, length: usize, random: &mut StdRng) -> Bytes {
        let mut bytes = vec![0u8; length];
        random.fill(&mut bytes[..]);
        bytes.into()
    }

    fn random_to(&self, random: &mut StdRng) -> Address {
        let mut addr = [0u8; 20];
        random.fill(&mut addr);
        Address::from(addr)
    }

    fn random_gas_price(&self, random: &mut StdRng) -> u128 {
        random.random::<u128>()
    }

    fn random_gas(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_u256(&self, random: &mut StdRng) -> U256 {
        let mut bytes = [0u8; 32];
        random.fill(&mut bytes);

        U256::from_be_slice(&bytes)
    }

    fn random_input(&self, random: &mut StdRng) -> TransactionInput {
        let length = random.random_range(0..=MAX_INPUT_LENGTH);
        TransactionInput::new(self.random_bytes(length, random))
    }

    fn random_nonce(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_chain_id(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }
}

#[tokio::test]
async fn test_legacy_transaction_runner() {
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
    let runner = LegacyTransactionRunner::new(
        "http://localhost:8545".to_string(),
        SigningKey::from_slice(
            &hex::decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_legacy_transaction(&mut rng, &provider, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
