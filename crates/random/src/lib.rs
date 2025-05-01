use std::time::Duration;

use alloy::{
    consensus::BlobTransactionSidecar,
    eips::{eip4844::BYTES_PER_BLOB, eip7702::SignedAuthorization},
    primitives::{Address, Bytes, FixedBytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{AccessList, AccessListItem, Authorization, TransactionInput, TransactionRequest},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{
    constants::{
        MAX_ACCESS_LIST_LENGTH, MAX_ACCESSED_KEYS_LENGTH, MAX_AUTHORIZATION_LIST_LENGTH,
        MAX_BLOB_SIDECAR_LENGTH, MAX_BLOB_VERSIONED_HASHES_LENGTH, MAX_INPUT_LENGTH,
        MAX_TRANSACTION_TYPE,
    },
    errors::Error,
};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};
use tokio_util::sync::CancellationToken;

pub struct RandomTransactionRunner {
    rpc_url: String,
    sk: SigningKey,
    seed: u64,
}

impl RandomTransactionRunner {
    pub fn new(rpc_url: String, sk: SigningKey, seed: u64) -> Self {
        Self { rpc_url, sk, seed }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<(), Error> {
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
                    let to = if random_bool(0.5) {
                        if random_bool(0.5) {
                            TxKind::Create
                        } else {
                        TxKind::Call(self.random_to(&mut random))
                        }
                    } else {
                        if random_bool(0.5) { TxKind::Create } else { TxKind::Call(Address::ZERO) }
                    };

                    let gas_price = if random_bool(0.5) {
                        self.random_gas_price(&mut random)
                    } else {
                        provider.get_gas_price().await.unwrap()
                    };

                    let max_fee_per_gas = if random_bool(0.5) {
                        self.random_max_fee_per_gas(&mut random)
                    } else {
                        0 // TODO
                    };

                    let max_priority_fee_per_gas = if random_bool(0.5) {
                        self.random_max_priority_fee_per_gas(&mut random)
                    } else {
                        provider.get_max_priority_fee_per_gas().await.unwrap()
                    };

                    let max_fee_per_blob_gas =
                        if random_bool(0.5) { self.random_max_fee_per_blob_gas(&mut random) } else { 0 };

                    let gas = if random_bool(0.5) {
                        self.random_gas(&mut random)
                    } else {
                        0 // TODO
                    };

                    let value = if random_bool(0.5) {
                        self.random_u256(&mut random)
                    } else {
                        provider.get_account(sender).await.unwrap().balance / U256::from(100_000_000)
                    };

                    let input = if random_bool(0.5) {
                        self.random_input(&mut random)
                    } else {
                        TransactionInput::from(vec![])
                    };

                    let nonce = if random_bool(0.5) {
                        self.random_nonce(&mut random)
                    } else {
                        provider.get_account(sender).await.unwrap().nonce
                    };

                    let chain_id = if random_bool(0.5) {
                        self.random_chain_id(&mut random)
                    } else {
                        provider.get_chain_id().await.unwrap()
                    };

                    let access_list = if random_bool(0.5) {
                        self.random_access_list(&mut random)
                    } else {
                        AccessList::from(vec![])
                    };

                    let transaction_type = if random_bool(0.5) {
                        self.random_transaction_type(&mut random)
                    } else {
                        0 // [nethoxa] should we default to this
                    };

                    let blob_versioned_hashes =
                        if random_bool(0.5) { self.random_blob_versioned_hashes(&mut random) } else { vec![] };

                    let sidecar = if random_bool(0.5) {
                        self.random_sidecar(&mut random)
                    } else {
                        BlobTransactionSidecar::new(vec![], vec![], vec![])
                    };

                    let authorization_list =
                        if random_bool(0.5) { self.random_authorization_list(&mut random) } else { vec![] };

                    let tx = TransactionRequest {
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
                    };

                    let _ = provider.send_transaction(tx).await.unwrap();
                    tokio::time::sleep(Duration::from_millis(10)).await;
                } => {}
            }
        }

        Ok(())
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

    fn random_max_fee_per_gas(&self, random: &mut StdRng) -> u128 {
        random.random::<u128>()
    }

    fn random_max_priority_fee_per_gas(&self, random: &mut StdRng) -> u128 {
        random.random::<u128>()
    }

    fn random_max_fee_per_blob_gas(&self, random: &mut StdRng) -> u128 {
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

    fn random_u8(&self, random: &mut StdRng) -> u8 {
        random.random_range(0..=u8::MAX)
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

    fn random_transaction_type(&self, random: &mut StdRng) -> u8 {
        random.random_range(0..MAX_TRANSACTION_TYPE)
    }

    fn random_blob_versioned_hashes(&self, random: &mut StdRng) -> Vec<FixedBytes<32>> {
        let length = random.random_range(0..=MAX_BLOB_VERSIONED_HASHES_LENGTH);
        let mut hashes = vec![];

        for _ in 0..length {
            let bytes = self.random_bytes(32, random);
            let mut array: [u8; 32] = [0u8; 32];

            for i in 0..bytes.len() {
                array[i] = bytes[i];
            }

            let hash = FixedBytes::new(array);
            hashes.push(hash);
        }

        hashes
    }

    fn random_sidecar(&self, random: &mut StdRng) -> BlobTransactionSidecar {
        let same_length = random_bool(0.75); // [nethoxa] check as != length should not be the common case
        if same_length {
            let length = random.random_range(0..MAX_BLOB_SIDECAR_LENGTH);
            let mut blobs = vec![];
            let mut commitments = vec![];
            let mut proofs = vec![];

            for _ in 0..length {
                let bytes = self.random_bytes(BYTES_PER_BLOB, random);
                let mut array: [u8; BYTES_PER_BLOB] = [0u8; BYTES_PER_BLOB];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let blob = FixedBytes::new(array);
                blobs.push(blob);

                let bytes = self.random_bytes(48, random);
                let mut array: [u8; 48] = [0u8; 48];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let commitment = FixedBytes::new(array);
                commitments.push(commitment);

                let bytes = self.random_bytes(48, random);
                let mut array: [u8; 48] = [0u8; 48];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let proof = FixedBytes::new(array);
                proofs.push(proof);
            }

            BlobTransactionSidecar { blobs, commitments, proofs }
        } else {
            let blobs_length = random.random_range(0..MAX_BLOB_SIDECAR_LENGTH);
            let commitments_length = random.random_range(0..MAX_BLOB_SIDECAR_LENGTH);
            let proofs_length = random.random_range(0..MAX_BLOB_SIDECAR_LENGTH);

            let mut blobs = vec![];
            for _ in 0..blobs_length {
                let bytes = self.random_bytes(BYTES_PER_BLOB, random);
                let mut array: [u8; BYTES_PER_BLOB] = [0u8; BYTES_PER_BLOB];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let blob = FixedBytes::new(array);
                blobs.push(blob);
            }

            let mut commitments = vec![];
            for _ in 0..commitments_length {
                let bytes = self.random_bytes(48, random);
                let mut array: [u8; 48] = [0u8; 48];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let commitment = FixedBytes::new(array);
                commitments.push(commitment);
            }

            let mut proofs = vec![];
            for _ in 0..proofs_length {
                let bytes = self.random_bytes(48, random);
                let mut array: [u8; 48] = [0u8; 48];

                for i in 0..bytes.len() {
                    array[i] = bytes[i];
                }

                let proof = FixedBytes::new(array);
                proofs.push(proof);
            }

            BlobTransactionSidecar { blobs, commitments, proofs }
        }
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
}
