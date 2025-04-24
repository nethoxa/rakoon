pub mod config;
pub mod errors;

use std::{thread::sleep, time::Duration};

use alloy::{
    consensus::{SignableTransaction, TxLegacy},
    primitives::{Address, TxKind, U256},
    providers::Provider,
    signers::{Signature, k256::ecdsa::SigningKey},
};
use rand::{Rng, RngCore};

use crate::{config::Config, errors::SpammerError};

/// Spammer holds the logic for sending arbitrary transactions.
pub struct Spammer {
    /// The config for the spammer.
    config: Config,
}

impl Spammer {
    /// Create a new `Spammer` with the given config.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Send legacy transactions.
    pub async fn send_legacy_txs(&self, key: &SigningKey) -> Result<(), SpammerError> {
        let from = Address::from_public_key(key.verifying_key());

        // Get the chain ID
        let chain_id = self
            .config
            .backend
            .get_chain_id()
            .await
            .map_err(|e| SpammerError::ProviderError(e.to_string()))?;

        // Get the gas price
        let gas_price = self
            .config
            .backend
            .get_gas_price()
            .await
            .map_err(|e| SpammerError::ProviderError(e.to_string()))?;

        // Generate random code
        let code = {
            // [nethoxa] TODO create valid code
            let mut rng = rand::rng();
            let length = rng.random_range(0..=128);
            let mut bytes = vec![0u8; length];
            rng.fill_bytes(&mut bytes);
            bytes
        };

        // Generate a random `to` address
        let to = {
            let mut rng = rand::rng();
            let mut bytes = vec![0u8; 20];
            rng.fill_bytes(&mut bytes);
            Address::from_slice(&bytes)
        };

        for i in 0..self.config.tx_number {
            // Get the account -> nonce, as we do not make assumptions about the nonce
            let account = self
                .config
                .backend
                .get_account(from)
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))?;

            // Create the transaction
            let tx = TxLegacy {
                chain_id: Some(chain_id),
                nonce: account.nonce,
                gas_price,
                gas_limit: 1000000, // [nethoxa] TODO: make this dynamic
                to: TxKind::Call(to),
                value: U256::from(i % 2), // [nethoxa] TODO: make this dynamic
                input: code.clone().into(),
            };

            // Sign the transaction at the bytecode level
            let tx_encoded = tx.encoded_for_signing();
            let (signature, recovery_id) = key
                .sign_recoverable(tx_encoded.as_slice())
                .map_err(|e| SpammerError::SigningError(e.to_string()))?;
            let tx_signed = tx.into_signed(Signature::from_bytes_and_parity(
                signature.to_bytes().as_slice(),
                recovery_id.to_byte() == 0,
            )); // [nethoxa] TODO: check if this is correct

            // Encode the transaction
            let mut buffer = vec![];
            tx_signed.rlp_encode(&mut buffer);

            // Send the transaction
            let _ = self
                .config
                .backend
                .send_raw_transaction(buffer.as_slice())
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))?;

            // Wait a bit to not saturate the network
            sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    /// Send blob transactions.
    pub async fn send_blob_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }

    /// Send 7702 transactions.
    pub async fn send_7702_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }
}
