use crate::errors::SpammerError;
use alloy::{
    eips::eip1898::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{Backend, SK, STATIC_KEYS};
use mutator::Mutator;

#[derive(Clone)]
/// Config is a struct that contains the configuration for the spammer.
pub struct Config {
    /// The backend to use for the spammer.
    pub backend: Backend,
    /// The number of transactions to send.
    pub tx_number: u64,
    /// The faucet account.
    pub faucet: SigningKey,
    /// The keys to send transactions from.
    pub keys: Vec<SigningKey>,
    /// The corpus to use for the spammer.
    pub corpus: Vec<Vec<u8>>,
    /// Whether to use access list or not.
    pub access_list: bool,
    /// The gas limit for the spammer.
    pub gas_limit: u64,
    /// The slot time for the spammer.
    pub slot_time: u64,
    /// The seed for the mutator.
    pub seed: u64,
    /// The mutator to use for the spammer.
    pub mutator: Mutator,
}

impl Config {
    /// Creates a new `Config` with the default values.
    pub fn default(
        rpc: String,
        tx_number: u64,
        access_list: bool,
        max_operations_per_mutation: u64,
    ) -> Result<Self, SpammerError> {
        let faucet = SigningKey::from_bytes(SK.as_bytes().into()).unwrap();
        let rpc_url = rpc.parse().unwrap();
        let backend = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(faucet.clone().into())
            .connect_http(rpc_url);

        let mut keys = Vec::new();
        for i in 0..STATIC_KEYS.len() {
            let key = SigningKey::from_bytes(STATIC_KEYS[i].as_bytes().into()).unwrap();
            keys.push(key);
        }

        Ok(Self {
            backend,
            tx_number,
            faucet,
            keys,
            corpus: Vec::new(),
            access_list,
            gas_limit: 30_000_000,
            slot_time: 12,
            seed: 0,
            mutator: Mutator::new(max_operations_per_mutation, 0),
        })
    }

    pub async fn new(
        rpc: String,
        sk: String,
        gas_limit: u64,
        tx_number: u64,
        corpus_file: String,
        seed: u64,
        access_list: bool,
        max_operations_per_mutation: u64,
    ) -> Result<Self, SpammerError> {
        let faucet = SigningKey::from_bytes(SK.as_bytes().into()).unwrap();
        let rpc_url = rpc.parse().unwrap();
        let backend = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(faucet.clone().into())
            .connect_http(rpc_url);

        let faucet = if sk.is_empty() {
            SigningKey::from_bytes(SK.as_bytes().into()).unwrap()
        } else {
            SigningKey::from_bytes(sk.as_bytes().into()).unwrap()
        };

        let mut keys = Vec::new();
        for i in 0..STATIC_KEYS.len() {
            let key = SigningKey::from_bytes(STATIC_KEYS[i].as_bytes().into()).unwrap();
            keys.push(key);
        }

        let gas_limit = if gas_limit == 0 {
            backend
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await
                .unwrap()
                .unwrap()
                .header
                .gas_limit
        } else {
            gas_limit
        };

        let tx_number: u64 = if tx_number == 0 {
            Self::setup_n(&backend, keys.len() as u64, gas_limit).await.unwrap()
        } else {
            tx_number
        };

        let corpus = if corpus_file.is_empty() {
            Vec::new()
        } else {
            Self::read_corpus(corpus_file).unwrap()
        };

        Ok(Self {
            backend,
            tx_number,
            faucet,
            keys,
            corpus,
            access_list,
            gas_limit,
            slot_time: 12,
            seed,
            mutator: Mutator::new(max_operations_per_mutation, seed),
        })
    }

    async fn setup_n(backend: &Backend, keys: u64, gas_limit: u64) -> Result<u64, SpammerError> {
        let header =
            backend.get_block_by_number(BlockNumberOrTag::Latest).await.unwrap().unwrap().header;
        let tx_per_block = header.gas_limit / gas_limit;
        let tx_per_account = tx_per_block / keys;
        if tx_per_account == 0 {
            return Ok(1);
        }
        Ok(tx_per_account)
    }

    fn read_corpus(path: String) -> Result<Vec<Vec<u8>>, SpammerError> {
        let stats = std::fs::read_dir(path).unwrap();
        let mut corpus = Vec::new();
        for stat in stats {
            let file = std::fs::read(stat.unwrap().path())
                .map_err(|e| SpammerError::FailedToReadCorpus(e.to_string()))?;
            corpus.push(file);
        }
        Ok(corpus)
    }
}
