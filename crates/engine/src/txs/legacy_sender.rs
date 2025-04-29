use alloy::{
    primitives::{Address, Bytes, TxKind, U256},
    providers::Provider,
    rpc::types::{TransactionInput, TransactionRequest},
};
use common::{Backend, MAX_GAS_LIMIT};
use rand::{Rng, RngCore, SeedableRng, random_bool, rngs::StdRng};

pub trait LegacySender {
    async fn create_legacy_tx(&self, sender: Address) -> TransactionRequest {
        let mut random = StdRng::seed_from_u64(self.seed());

        let chain_id = if random_bool(0.5) {
            self.random_chain_id(&mut random)
        } else {
            self.get_chain_id().await
        };

        let nonce = if random_bool(0.5) {
            self.random_nonce(&mut random)
        } else {
            self.get_nonce(sender).await
        };

        let gas_price = if random_bool(0.5) {
            self.random_gas_price(&mut random)
        } else {
            self.get_gas_price().await
        };

        let gas_limit =
            if random_bool(0.5) { self.random_gas_limit(&mut random) } else { MAX_GAS_LIMIT };

        let to = if random_bool(0.5) {
            if random_bool(0.5) {
                TxKind::Create
            } else {
                TxKind::Call(self.random_to(&mut random))
            }
        } else {
            if random_bool(0.5) { TxKind::Create } else { TxKind::Call(Address::ZERO) }
        };

        let value = if random_bool(0.5) {
            self.random_u256(&mut random)
        } else {
            self.get_value(sender).await
        };

        let input = if random_bool(0.5) {
            self.random_input(&mut random)
        } else {
            TransactionInput::from(vec![])
        };

        TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            max_fee_per_blob_gas: None,
            gas: None,
            value: Some(value),
            input,
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: None,
            transaction_type: None,
            blob_versioned_hashes: None,
            sidecar: None,
            authorization_list: None,
        }
    }

    fn random_to(&self, random: &mut StdRng) -> Address {
        let mut addr = [0u8; 20];
        random.fill(&mut addr);
        Address::from(addr)
    }

    fn random_chain_id(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_nonce(&self, random: &mut StdRng) -> u64 {
        random.next_u64()
    }

    fn random_gas_price(&self, random: &mut StdRng) -> u128 {
        random.random::<u128>()
    }

    fn random_gas_limit(&self, random: &mut StdRng) -> u64 {
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

    fn random_bytes(&self, length: usize, random: &mut StdRng) -> Bytes {
        let mut bytes = vec![0u8; length];
        random.fill(&mut bytes[..]);
        bytes.into()
    }

    // ------------------------------------

    async fn get_chain_id(&self) -> u64 {
        self.get_backend().get_chain_id().await.unwrap()
    }

    async fn get_value(&self, address: Address) -> U256 {
        self.get_backend().get_account(address).await.unwrap().balance / self.max_balance_divisor()
    }

    async fn get_nonce(&self, address: Address) -> u64 {
        self.get_backend().get_account(address).await.unwrap().nonce
    }

    async fn get_gas_price(&self) -> u128 {
        self.get_backend().get_gas_price().await.unwrap()
    }

    fn seed(&self) -> u64;
    fn get_backend(&self) -> &Backend;
    fn max_balance_divisor(&self) -> U256;
    fn max_input_length(&self) -> usize;
}
