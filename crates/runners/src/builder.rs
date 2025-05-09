use std::{fs::OpenOptions, io::Write};

use alloy::{
    consensus::BlobTransactionSidecar,
    eips::{eip4844::BYTES_PER_BLOB, eip7702::SignedAuthorization},
    primitives::{Address, Bytes, FixedBytes, TxKind, U256},
    providers::Provider,
    rpc::types::{AccessList, AccessListItem, Authorization, TransactionInput}, transports::{RpcError, TransportErrorKind},
};
use common::{
    constants::{
        MAX_ACCESS_LIST_LENGTH, MAX_ACCESSED_KEYS_LENGTH, MAX_AUTHORIZATION_LIST_LENGTH,
        MAX_BLOB_SIDECAR_LENGTH, MAX_BLOB_VERSIONED_HASHES_LENGTH, MAX_GAS_LIMIT, MAX_INPUT_LENGTH,
        MAX_TRANSACTION_TYPE,
    },
    types::Backend,
};
use rand::{Rng, RngCore, random_bool, rngs::StdRng};

use crate::logger::Logger;

pub trait Builder {
    fn provider(&self) -> &Backend;
    fn is_running(&self) -> bool;
    fn crash_counter(&self) -> u64;
    fn logger(&mut self) -> &mut Logger;

    // ------------------------------------------------------------

    fn to(&self, random: &mut StdRng) -> TxKind {
        if random_bool(0.5) {
            TxKind::Create
        } else {
            TxKind::Call({
                let mut addr = [0u8; 20];
                random.fill(&mut addr);
                Address::from(addr)
            })
        }
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn gas_price(&self, random: &mut StdRng) -> u128 {
        if random_bool(0.85) {
            if let Ok(price) = self.provider().get_gas_price().await {
                price
            } else {
                random.random::<u128>()
            }
        } else {
            random.random::<u128>()
        }
    }

    // ------------------------------------------------------------

    // [nethoxa] this should be better implemented
    fn max_fee_per_gas(&self, random: &mut StdRng) -> u128 {
        random.random::<u128>()
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn max_priority_fee_per_gas(&self, random: &mut StdRng) -> u128 {
        if random_bool(0.85) {
            if let Ok(fee) = self.provider().get_max_priority_fee_per_gas().await {
                fee
            } else {
                random.random::<u128>()
            }
        } else {
            random.random::<u128>()
        }
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn max_fee_per_blob_gas(&self, random: &mut StdRng) -> u128 {
        if random_bool(0.85) {
            if let Ok(fee) = self.provider().get_blob_base_fee().await {
                fee
            } else {
                random.random::<u128>()
            }
        } else {
            random.random::<u128>()
        }
    }

    // ------------------------------------------------------------

    // [nethoxa] should implement a call to gas estimation
    fn gas(&self, random: &mut StdRng) -> u64 {
        random.random_range(0..=MAX_GAS_LIMIT * 2)
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn value(&self, random: &mut StdRng, sender: Address) -> U256 {
        if random_bool(0.85) {
            if let Ok(balance) = self.provider().get_account(sender).await {
                balance.balance / U256::from(100_000_000)
            } else {
                self.random_u256(random)
            }
        } else {
            self.random_u256(random)
        }
    }

    fn random_u256(&self, random: &mut StdRng) -> U256 {
        let mut bytes = [0u8; 32];
        random.fill(&mut bytes);
        U256::from_be_slice(&bytes)
    }

    // ------------------------------------------------------------

    fn input(&self, random: &mut StdRng) -> TransactionInput {
        if random_bool(0.2) {
            let length = random.random_range(0..=MAX_INPUT_LENGTH);
            TransactionInput::new(self.random_bytes(length, random))
        } else {
            TransactionInput::from(vec![])
        }
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn nonce(&self, random: &mut StdRng, sender: Address) -> u64 {
        if random_bool(0.85) {
            if let Ok(nonce) = self.provider().get_account(sender).await {
                nonce.nonce
            } else {
                random.next_u64()
            }
        } else {
            random.next_u64()
        }
    }

    // ------------------------------------------------------------

    #[allow(async_fn_in_trait)]
    async fn chain_id(&self, random: &mut StdRng) -> u64 {
        if random_bool(0.85) {
            if let Ok(chain_id) = self.provider().get_chain_id().await {
                chain_id
            } else {
                random.next_u64()
            }
        } else {
            random.next_u64()
        }
    }

    // ------------------------------------------------------------

    fn access_list(&self, random: &mut StdRng) -> AccessList {
        if random_bool(0.2) { self.random_access_list(random) } else { AccessList::from(vec![]) }
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

    // ------------------------------------------------------------

    fn transaction_type(&self, random: &mut StdRng) -> u8 {
        // [nethoxa] should we send tx with wrong transaction type?
        random.random_range(0..MAX_TRANSACTION_TYPE)
    }

    // ------------------------------------------------------------

    fn blob_versioned_hashes(&self, random: &mut StdRng) -> Vec<FixedBytes<32>> {
        if random_bool(0.2) { self.random_blob_versioned_hashes(random) } else { vec![] }
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

    // ------------------------------------------------------------

    fn sidecar(&self, random: &mut StdRng) -> BlobTransactionSidecar {
        if random_bool(0.2) {
            self.random_sidecar(random)
        } else {
            BlobTransactionSidecar::new(vec![], vec![], vec![])
        }
    }

    fn random_sidecar(&self, random: &mut StdRng) -> BlobTransactionSidecar {
        let same_length = random_bool(0.75);
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

    // ------------------------------------------------------------

    fn authorization_list(&self, random: &mut StdRng) -> Vec<SignedAuthorization> {
        if random_bool(0.2) { self.random_authorization_list(random) } else { vec![] }
    }

    fn random_authorization_list(&self, random: &mut StdRng) -> Vec<SignedAuthorization> {
        let length = random.random_range(0..=MAX_AUTHORIZATION_LIST_LENGTH);
        let mut authorizations = vec![];

        for _ in 0..length {
            let chain_id = self.random_u256(random);
            let addr = self.random_address(random);
            let nonce = random.next_u64();

            let auth = Authorization { chain_id, address: addr, nonce };

            let y_parity = random.random::<u8>();
            let r = self.random_u256(random);
            let s = self.random_u256(random);

            let signed = SignedAuthorization::new_unchecked(auth, y_parity, r, s);

            authorizations.push(signed);
        }

        authorizations
    }

    // ------------------------------------------------------------

    fn random_bytes(&self, length: usize, random: &mut StdRng) -> Bytes {
        let mut bytes = vec![0u8; length];
        random.fill(&mut bytes[..]);
        bytes.into()
    }

    fn random_address(&self, random: &mut StdRng) -> Address {
        let mut addr = [0u8; 20];
        random.fill(&mut addr);
        Address::from(addr)
    }

    // ------------------------------------------------------------

    fn is_connection_refused_error(err: &RpcError<TransportErrorKind>) -> bool {
        let formatted_err = format!("{:#?}", err);
        formatted_err.contains("Connection refused")
    }

    #[allow(async_fn_in_trait)]
    async fn generate_crash_report(&mut self, tx_bytes: &[u8]) {
        let report = format!(
            "Transaction bytes (hex): 0x{}\n",
            hex::encode(tx_bytes)
        );
        
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("crash_report_legacy_{}.log", self.crash_counter())) 
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(report.as_bytes()) {
                    let _ = self.logger().log_error(&format!("Failed to write crash report to file: {}", e));
                }
                if let Err(e) = file.flush() {
                    let _ = self.logger().log_error(&format!("Failed to flush crash report file: {}", e));
                }
            }
            Err(e) => {
                let _ = self.logger().log_error(&format!("Failed to open crash report file: {}", e));
            }
        }
    }
}
