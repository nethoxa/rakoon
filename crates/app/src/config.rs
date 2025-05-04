use alloy::signers::k256::ecdsa::SigningKey;
use std::collections::HashMap;

pub struct Config {
    pub rpc_url: String,
    pub sk: SigningKey,
    pub seed: u64,
    pub random_enabled: bool,
    pub legacy_enabled: bool,
    pub al_enabled: bool,
    pub blob_enabled: bool,
    pub eip1559_enabled: bool,
    pub eip7702_enabled: bool,
    // Global configuration
    pub global_seed: Option<u64>,
    pub global_sk: Option<SigningKey>,
    // Per-runner configuration
    pub runner_seeds: HashMap<String, u64>,
    pub runner_sks: HashMap<String, SigningKey>,
    // Active runners
    pub active_runners: HashMap<String, bool>,
}

impl Config {
    pub fn new(
        rpc_url: String,
        sk: SigningKey,
        seed: u64,
        random_enabled: bool,
        legacy_enabled: bool,
        al_enabled: bool,
        blob_enabled: bool,
        eip1559_enabled: bool,
        eip7702_enabled: bool,
    ) -> Self {
        Self {
            rpc_url,
            sk,
            seed,
            random_enabled,
            legacy_enabled,
            al_enabled,
            blob_enabled,
            eip1559_enabled,
            eip7702_enabled,
            global_seed: None,
            global_sk: None,
            runner_seeds: HashMap::new(),
            runner_sks: HashMap::new(),
            active_runners: HashMap::new(),
        }
    }

    pub fn set_rpc_url(&mut self, rpc_url: String) {
        self.rpc_url = rpc_url;
    }

    pub fn set_sk(&mut self, sk: SigningKey) {
        self.sk = sk;
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    pub fn set_random_enabled(&mut self, enabled: bool) {
        self.random_enabled = enabled;
    }

    pub fn set_legacy_enabled(&mut self, enabled: bool) {
        self.legacy_enabled = enabled;
    }

    pub fn set_al_enabled(&mut self, enabled: bool) {
        self.al_enabled = enabled;
    }

    pub fn set_blob_enabled(&mut self, enabled: bool) {
        self.blob_enabled = enabled;
    }

    pub fn set_eip1559_enabled(&mut self, enabled: bool) {
        self.eip1559_enabled = enabled;
    }

    pub fn set_eip7702_enabled(&mut self, enabled: bool) {
        self.eip7702_enabled = enabled;
    }

    pub fn set_global_seed(&mut self, seed: u64) {
        self.global_seed = Some(seed);
    }

    pub fn set_global_sk(&mut self, sk: SigningKey) {
        self.global_sk = Some(sk);
    }

    pub fn set_runner_seed(&mut self, runner: String, seed: u64) {
        self.runner_seeds.insert(runner, seed);
    }

    pub fn set_runner_sk(&mut self, runner: String, sk: SigningKey) {
        self.runner_sks.insert(runner, sk);
    }

    pub fn get_runner_seed(&self, runner: &str) -> u64 {
        self.runner_seeds.get(runner)
            .copied()
            .or(self.global_seed)
            .unwrap_or(self.seed)
    }

    pub fn get_runner_sk(&self, runner: &str) -> &SigningKey {
        self.runner_sks.get(runner)
            .or(self.global_sk.as_ref())
            .unwrap_or(&self.sk)
    }

    pub fn start_runner(&mut self, runner: String) {
        self.active_runners.insert(runner, true);
    }

    pub fn stop_runner(&mut self, runner: &str) {
        self.active_runners.remove(runner);
    }

    pub fn stop_all_runners(&mut self) {
        self.active_runners.clear();
    }

    pub fn is_runner_active(&self, runner: &str) -> bool {
        self.active_runners.get(runner).copied().unwrap_or(false)
    }
}
