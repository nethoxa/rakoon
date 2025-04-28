use core::slice::SlicePattern;

use alloy::{
    consensus::BlobTransactionSidecar, eips::eip7702::SignedAuthorization, primitives::{Address, Bytes, FixedBytes, TxKind, U256}, providers::Provider, rpc::types::{AccessList, AccessListItem, TransactionInput, TransactionRequest}, signers::k256::{ecdsa::SigningKey, elliptic_curve::bigint::Random}
};
use common::Backend;
use mutator::Mutator;
use rand::{random_bool, rngs::StdRng, Rng, RngCore, SeedableRng};

pub trait RandomSender {
    async fn create_random_tx(&self, sender: Address, key: SigningKey) -> Vec<u8> {
        let backend = self.get_backend();
        let mut rng = StdRng::seed_from_u64(self.seed());

        let mut tx = TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            max_fee_per_blob_gas,
            gas,
            value,
            input,
            nonce: nonce_value,
            chain_id,
            access_list,
            transaction_type,
            blob_versioned_hashes,
            sidecar,
            authorization_list,
        };

        todo!()
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

    fn random_input(&self, random: &mut StdRng) -> TransactionInput {
        let length = random.random_range(0..=self.max_input_length());
        TransactionInput::new(self.random_bytes(length, random))
    }

    fn random_nonce(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_chain_id(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_access_list(&self, random: &mut StdRng) -> AccessList {
        let length = random.random_range(0..=self.max_access_list_length());
        let mut items = vec![];

        for _ in 0..length {
            let addr = self.random_address(random);

            let keys_length = random.random_range(0..=self.max_accessed_keys_length())
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

            let item = AccessListItem {
                address: addr,
                storage_keys: keys
            };
            items.push(item);
        }

        AccessList(items)
    }

    fn random_transaction_type(&self, random: &mut StdRng) -> u8 {
        random.random_range(0..self.max_transaction_type())
    }

    fn random_blob_versioned_hashes(&self, random: &mut StdRng) -> Vec<FixedBytes<32>> {
        let length = random.random_range(0..=self.max_blob_versioned_hashes_length());
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
        
    }

    fn random_authorization_list(&self, random: &mut StdRng) -> Vec<SignedAuthorization> {

    }





    fn get_backend(&self) -> Backend;
    fn current_tx(&self) -> &[u8];
    fn current_tx_mut(&self) -> &mut [u8];
    fn seed(&self) -> u64;
    fn max_operations_per_mutation(&self) -> u64;
    fn max_input_length(&self) -> usize;
    fn max_access_list_length(&self) -> usize;
    fn max_accessed_keys_length(&self) -> usize;
    fn max_transaction_type(&self) -> u8; // [nethoxa] parametrizable
    fn max_blob_versioned_hashes_length(&self) -> usize;
}
