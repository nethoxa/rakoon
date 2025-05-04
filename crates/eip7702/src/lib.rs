use alloy::{
    consensus::TxEip7702,
    eips::eip7702::SignedAuthorization,
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
        MAX_INPUT_LENGTH,
    },
    errors::Error,
};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub struct EIP7702TransactionRunner {
    rpc_url: String,
    sk: SigningKey,
    seed: u64,
    tx_sent: u64,
}

impl EIP7702TransactionRunner {
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
                    let tx = self.create_eip7702_transaction(&mut random, &provider, sender).await;
                    let _ = provider.send_transaction(tx).await.unwrap();
                    self.tx_sent += 1;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                } => {}
            }
        }

        Ok(())
    }

    pub async fn create_eip7702_transaction(
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

        let max_fee_per_gas = if random_bool(0.5) {
            self.random_gas_price(random)
        } else {
            provider.get_gas_price().await.unwrap()
        };

        let max_priority_fee_per_gas = if random_bool(0.5) {
            self.random_gas_price(random)
        } else {
            provider.get_max_priority_fee_per_gas().await.unwrap()
        };

        let gas_limit = if random_bool(0.5) {
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

        let access_list =
            if random_bool(0.5) { self.random_access_list(random) } else { AccessList::default() };

        let authorization_list =
            if random_bool(0.5) { self.random_authorization_list(random) } else { vec![] };

        // EIP-7702 transaction type
        let transaction_type = 5;

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
            authorization_list: Some(authorization_list),
        }
    }

    fn random_access_list(&self, random: &mut StdRng) -> AccessList {
        let length = random.random_range(0..=MAX_ACCESS_LIST_LENGTH);
        let mut items = vec![];

        for _ in 0..length {
            let addr = self.random_address(random);

            let keys_length = random.random_range(0..=MAX_ACCESSED_KEYS_LENGTH);
            let mut keys: Vec<FixedBytes<32>> = vec![];

            for _ in 0..keys_length {
                let bytes = self.random_bytes(32, random);
                let mut array: [u8; 32] = [0u8; 32];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let key = FixedBytes::new(array);
                keys.push(key);
            }

            let item = AccessListItem { address: addr, storage_keys: keys };
            items.push(item);
        }

        AccessList(items)
    }

    fn random_authorization_list(&self, random: &mut StdRng) -> Vec<SignedAuthorization> {
        let length = random.random_range(0..=MAX_AUTHORIZATION_LIST_LENGTH);
        let mut authorizations = vec![];

        for _ in 0..length {
            let chain_id = self.random_u256(random);
            let addr = self.random_address(random);
            let nonce = self.random_nonce(random);

            let auth = Authorization { chain_id, address: addr, nonce };

            let y_parity = self.random_u8(random);
            let r = self.random_u256(random);
            let s = self.random_u256(random);

            let signed = SignedAuthorization::new_unchecked(auth, y_parity, r, s);

            authorizations.push(signed);
        }

        authorizations
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

    fn random_address(&self, random: &mut StdRng) -> Address {
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

    fn random_u8(&self, random: &mut StdRng) -> u8 {
        random.random_range(0..=u8::MAX)
    }
}

#[tokio::test]
async fn test_eip7702_transaction_runner() {
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
    let runner = EIP7702TransactionRunner::new(
        "http://localhost:8545".to_string(),
        SigningKey::from_slice(
            &hex::decode("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                .unwrap(),
        )
        .unwrap(),
        1,
    );
    let tx = runner.create_eip7702_transaction(&mut rng, &provider, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
