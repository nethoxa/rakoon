use alloy::{primitives::{Address, U256}, providers::Provider};
use common::types::Backend;

pub struct BuilderCache {
    pub gas_price: u128,
    pub max_priority_fee: u128,
    pub max_fee_per_blob_gas: u128,
    pub balance: U256,
    pub nonce: u64,
    pub chain_id: u64,
}

impl BuilderCache {
    pub async fn update(&mut self, provider: &Backend, address: Address) {
        let account = provider.get_account(address).await.unwrap_or_default();

        self.gas_price = provider.get_gas_price().await.unwrap_or_default();
        self.max_priority_fee = provider.get_max_priority_fee_per_gas().await.unwrap_or_default();
        self.max_fee_per_blob_gas = provider.get_blob_base_fee().await.unwrap_or_default();
        self.balance = account.balance;
        self.nonce = account.nonce;
        self.chain_id = provider.get_chain_id().await.unwrap_or_default();
    }
}