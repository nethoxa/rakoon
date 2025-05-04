use alloy::signers::k256::ecdsa::SigningKey;

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
}
