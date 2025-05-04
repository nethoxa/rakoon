use std::str::FromStr;

use crate::{App, errors::AppStatus};
use alloy::{hex, signers::k256::ecdsa::SigningKey, transports::http::reqwest::Url};
use common::{constants::SK, parse_sk};
use runners::Runner::{self, *};

impl App {
    // Helper function to check if a runner type is valid
    fn is_valid_runner(&self, runner: &str) -> bool {
        [
            "al", "blob", "eip1559", "eip7702", "legacy", "random",
        ]
        .contains(&runner)
    }

    // Helper function to check if a scope is valid
    fn is_valid_scope(&self, scope: &str) -> bool {
        [
            "global", "al", "blob", "eip1559", "eip7702", "legacy", "random",
        ]
        .contains(&scope)
    }

    // Helper function to check if a parameter is valid
    fn is_valid_param(&self, param: &str) -> bool {
        [
            "rpc", "sk", "seed", "happy",
        ]
        .contains(&param)
    }

    // Helper function to parse a boolean
    fn parse_bool(&self, value: &str) -> Result<bool, AppStatus> {
        match value {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(AppStatus::RuntimeError),
        }
    }

    // Helper function to handle setting globalconfig values
    fn handle_global_set(&mut self, param: &str, value: &str) -> Result<(), AppStatus> {
        match param {
            "rpc" => {
                if let Ok(url) = Url::parse(value) {
                    if self.rpc_url == url {
                        self.print(&format!("global rpc url already set to that value"));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.rpc_url = url;
                    self.print(&format!("global rpc url set to {}", value));
                } else {
                    self.print(&format!("invalid rpc url: {}", value));
                    return Err(AppStatus::RuntimeError);
                }
            }
            "sk" => {
                if let Ok(sk) = parse_sk(value) {
                    if self.sk == sk {
                        self.print(&format!("global sk already set to that value"));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.sk = sk;
                    self.print(&format!("global sk set to {}", value));
                } else {
                    self.print(&format!("invalid sk: {}", value));
                    return Err(AppStatus::RuntimeError);
                }
            }
            "seed" => {
                if let Ok(seed) = value.parse::<u64>() {
                    if self.seed == seed {
                        self.print(&format!("global seed already set to that value"));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.seed = seed;
                    self.print(&format!("global seed set to {}", value));
                } else {
                    self.print(&format!("invalid seed: {}", value));
                    return Err(AppStatus::RuntimeError);
                }
            }
            "happy" => {
                if let Ok(happy) = self.parse_bool(value) {
                    if self.happy == happy {
                        self.print(&format!("global happy already set to that value"));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.happy = happy;
                    self.print(&format!("global happy set to {}", value));
                } else {
                    self.print(&format!("invalid happy: {}", value));
                    return Err(AppStatus::RuntimeError);
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    // Helper function to handle setting per-runner config values
    fn handle_runner_set(
        &mut self,
        runner: &str,
        param: &str,
        value: &str,
    ) -> Result<(), AppStatus> {
        match runner {
            "al" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&AL) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(AL, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&AL) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(AL, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&AL) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(AL, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&AL) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(AL, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            "blob" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&Blob) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(Blob, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&Blob) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(Blob, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&Blob) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(Blob, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&Blob) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(Blob, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            "eip1559" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&EIP1559) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(EIP1559, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&EIP1559) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(EIP1559, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&EIP1559) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(EIP1559, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&EIP1559) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(EIP1559, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            "eip7702" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&EIP7702) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(EIP7702, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&EIP7702) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(EIP7702, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&EIP7702) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(EIP7702, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&EIP7702) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(EIP7702, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            "legacy" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&Legacy) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(Legacy, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&Legacy) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(Legacy, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&Legacy) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(Legacy, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&Legacy) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(Legacy, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            "random" => match param {
                "rpc" => {
                    if let Ok(url) = Url::parse(value) {
                        if self.runner_rpcs.get(&Random) == Some(&url) {
                            self.print(&format!("{} rpc url already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_rpcs.insert(Random, url);
                        self.print(&format!("{} rpc url set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid rpc url: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "sk" => {
                    if let Ok(sk) = parse_sk(value) {
                        if self.runner_sks.get(&Random) == Some(&sk) {
                            self.print(&format!("{} sk already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_sks.insert(Random, sk);
                        self.print(&format!("{} sk set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid sk: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "seed" => {
                    if let Ok(seed) = value.parse::<u64>() {
                        if self.runner_seeds.get(&Random) == Some(&seed) {
                            self.print(&format!("{} seed already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_seeds.insert(Random, seed);
                        self.print(&format!("{} seed set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid seed: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                "happy" => {
                    if let Ok(happy) = self.parse_bool(value) {
                        if self.runner_happy.get(&Random) == Some(&happy) {
                            self.print(&format!("{} happy already set to that value", runner));
                            return Err(AppStatus::RuntimeError);
                        }
                        self.runner_happy.insert(Random, happy);
                        self.print(&format!("{} happy set to {}", runner, value));
                    } else {
                        self.print(&format!("invalid happy: {}", value));
                        return Err(AppStatus::RuntimeError);
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        Ok(())
    }

    // Helper function to handle resetting global config
    fn handle_global_reset(&mut self, param: &str) -> Result<(), AppStatus> {
        match param {
            "all" => {
                let sk = SigningKey::from_slice(hex::decode(SK).unwrap().as_slice()).unwrap();
                let url = Url::parse("http://localhost:8545").unwrap();
                if self.rpc_url == url && self.seed == 0 && self.sk == sk && self.happy == false {
                    self.print("global config is already reset");
                    return Err(AppStatus::RuntimeError);
                }
                self.rpc_url = url;
                self.seed = 0;
                self.sk = sk;
                self.happy = false;
            }
            "rpc" => {
                let url = Url::parse("http://localhost:8545").unwrap();
                if self.rpc_url == url {
                    self.print("global rpc url is already reset");
                    return Err(AppStatus::RuntimeError);
                }
                self.rpc_url = url;
            }
            "sk" => {
                let sk = SigningKey::from_slice(hex::decode(SK).unwrap().as_slice()).unwrap();
                if self.sk == sk {
                    self.print("global sk is already reset");
                    return Err(AppStatus::RuntimeError);
                }
                self.sk = sk;
            }
            "seed" => {
                if self.seed == 0 {
                    self.print("global seed is already reset");
                    return Err(AppStatus::RuntimeError);
                }
                self.seed = 0;
            }
            "happy" => {
                if self.happy == false {
                    self.print("global happy is already reset");
                    return Err(AppStatus::RuntimeError);
                }
                self.happy = false;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn handle_runner_reset(&mut self, runner: &str, param: &str) -> Result<(), AppStatus> {
        match runner {
            "al" => match param {
                "all" => {
                    if self.runner_rpcs.get(&AL) == None
                        && self.runner_sks.get(&AL) == None
                        && self.runner_seeds.get(&AL) == None
                        && self.runner_happy.get(&AL) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&AL);
                    self.runner_sks.remove(&AL);
                    self.runner_seeds.remove(&AL);
                    self.runner_happy.remove(&AL);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&AL) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&AL);
                }
                "sk" => {
                    if self.runner_sks.get(&AL) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&AL);
                }
                "seed" => {
                    if self.runner_seeds.get(&AL) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&AL);
                }
                "happy" => {
                    if self.runner_happy.get(&AL) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&AL);
                }
                _ => unreachable!(),
            },
            "blob" => match param {
                "all" => {
                    if self.runner_rpcs.get(&Blob) == None
                        && self.runner_sks.get(&Blob) == None
                        && self.runner_seeds.get(&Blob) == None
                        && self.runner_happy.get(&Blob) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Blob);
                    self.runner_sks.remove(&Blob);
                    self.runner_seeds.remove(&Blob);
                    self.runner_happy.remove(&Blob);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&Blob) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Blob);
                }
                "sk" => {
                    if self.runner_sks.get(&Blob) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&Blob);
                }
                "seed" => {
                    if self.runner_seeds.get(&Blob) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&Blob);
                }
                "happy" => {
                    if self.runner_happy.get(&Blob) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&Blob);
                }
                _ => unreachable!(),
            },
            "eip1559" => match param {
                "all" => {
                    if self.runner_rpcs.get(&EIP1559) == None
                        && self.runner_sks.get(&EIP1559) == None
                        && self.runner_seeds.get(&EIP1559) == None
                        && self.runner_happy.get(&EIP1559) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&EIP1559);
                    self.runner_sks.remove(&EIP1559);
                    self.runner_seeds.remove(&EIP1559);
                    self.runner_happy.remove(&EIP1559);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&EIP1559) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&EIP1559);
                }
                "sk" => {
                    if self.runner_sks.get(&EIP1559) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&EIP1559);
                }
                "seed" => {
                    if self.runner_seeds.get(&EIP1559) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&EIP1559);
                }
                "happy" => {
                    if self.runner_happy.get(&EIP1559) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&EIP1559);
                }
                _ => unreachable!(),
            },
            "eip7702" => match param {
                "all" => {
                    if self.runner_rpcs.get(&EIP7702) == None
                        && self.runner_sks.get(&EIP7702) == None
                        && self.runner_seeds.get(&EIP7702) == None
                        && self.runner_happy.get(&EIP7702) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&EIP7702);
                    self.runner_sks.remove(&EIP7702);
                    self.runner_seeds.remove(&EIP7702);
                    self.runner_happy.remove(&EIP7702);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&EIP7702) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&EIP7702);
                }
                "sk" => {
                    if self.runner_sks.get(&EIP7702) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&EIP7702);
                }
                "seed" => {
                    if self.runner_seeds.get(&EIP7702) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&EIP7702);
                }
                "happy" => {
                    if self.runner_happy.get(&EIP7702) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&EIP7702);
                }
                _ => unreachable!(),
            },
            "legacy" => match param {
                "all" => {
                    if self.runner_rpcs.get(&Legacy) == None
                        && self.runner_sks.get(&Legacy) == None
                        && self.runner_seeds.get(&Legacy) == None
                        && self.runner_happy.get(&Legacy) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Legacy);
                    self.runner_sks.remove(&Legacy);
                    self.runner_seeds.remove(&Legacy);
                    self.runner_happy.remove(&Legacy);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&Legacy) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Legacy);
                }
                "sk" => {
                    if self.runner_sks.get(&Legacy) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&Legacy);
                }
                "seed" => {
                    if self.runner_seeds.get(&Legacy) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&Legacy);
                }
                "happy" => {
                    if self.runner_happy.get(&Legacy) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&Legacy);
                }
                _ => unreachable!(),
            },
            "random" => match param {
                "all" => {
                    if self.runner_rpcs.get(&Random) == None
                        && self.runner_sks.get(&Random) == None
                        && self.runner_seeds.get(&Random) == None
                        && self.runner_happy.get(&Random) == None
                    {
                        self.print(&format!("{} runner is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Random);
                    self.runner_sks.remove(&Random);
                    self.runner_seeds.remove(&Random);
                    self.runner_happy.remove(&Random);
                }
                "rpc" => {
                    if self.runner_rpcs.get(&Random) == None {
                        self.print(&format!("{} runner rpc is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_rpcs.remove(&Random);
                }
                "sk" => {
                    if self.runner_sks.get(&Random) == None {
                        self.print(&format!("{} runner sk is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_sks.remove(&Random);
                }
                "seed" => {
                    if self.runner_seeds.get(&Random) == None {
                        self.print(&format!("{} runner seed is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_seeds.remove(&Random);
                }
                "happy" => {
                    if self.runner_happy.get(&Random) == None {
                        self.print(&format!("{} runner happy is already reset", runner));
                        return Err(AppStatus::RuntimeError);
                    }
                    self.runner_happy.remove(&Random);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        Ok(())
    }

    // Helper function to handle commands
    pub async fn handle_command(&mut self, command: String) -> Result<(), AppStatus> {
        let command = command.trim();

        // Handle set command
        if command.starts_with("set ") {
            let parts: Vec<&str> = command.splitn(4, ' ').collect();
            if parts.len() == 4 {
                let scope = parts[1];
                let param = parts[2];
                let value = parts[3];

                if !self.is_valid_scope(scope) {
                    self.print(&format!("invalid scope: {}", scope));
                    return Err(AppStatus::RuntimeError);
                }

                if !self.is_valid_param(param) {
                    self.print(&format!("invalid parameter: {}", param));
                    return Err(AppStatus::RuntimeError);
                }

                match scope {
                    "global" => {
                        return self.handle_global_set(param, value);
                    }
                    _ => {
                        return self.handle_runner_set(scope, param, value);
                    }
                }
            }
        }

        // Handle reset command
        if command.starts_with("reset ") {
            let parts: Vec<&str> = command.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let scope = parts[1];
                let param = parts[2];

                if !self.is_valid_scope(scope) {
                    self.print(&format!("invalid scope: {}", scope));
                    return Err(AppStatus::RuntimeError);
                }

                if !self.is_valid_param(param) && param != "all" {
                    self.print(&format!("invalid parameter: {}", param));
                    return Err(AppStatus::RuntimeError);
                }

                if scope == "global" {
                    return self.handle_global_reset(param);
                } else if self.is_valid_runner(scope) {
                    return self.handle_runner_reset(scope, param);
                } else {
                    self.print(&format!("invalid scope: {}", scope));
                    return Err(AppStatus::RuntimeError);
                }
            } else {
                self.print("invalid reset command format. Use: reset <global/RUNNER> <RPC/sk/seed/happy/all>");
                return Err(AppStatus::RuntimeError);
            }
        }

        if command == "stop" {
            if self.active_runners.is_empty() {
                self.print("no runners to stop");
                return Err(AppStatus::RuntimeError);
            }

            let runners: Vec<_> = self.active_runners.keys().cloned().collect();
            for runner in runners {
                if let Err(e) = self.stop_runner(runner).await {
                    self.print(&format!("error stopping runner: {}", e));
                    return Err(AppStatus::RuntimeError);
                }
            }
            self.print("all runners stopped");
            return Ok(());
        }

        if command.starts_with("stop ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let runner = parts[1];
                if !self.is_valid_runner(runner) {
                    self.print(&format!("invalid runner: {}", runner));
                    return Err(AppStatus::RuntimeError);
                }

                if !self.active_runners.contains_key(&Runner::from_str(runner).unwrap()) {
                    self.print(&format!("{} runner is not active", runner));
                    return Err(AppStatus::RuntimeError);
                }

                if let Err(e) = self.stop_runner(Runner::from_str(runner).unwrap()).await {
                    self.print(&format!("error stopping runner: {}", e));
                    return Err(AppStatus::RuntimeError);
                }
                self.print(&format!("{} runner stopped", runner));
                return Ok(());
            } else {
                self.print("invalid stop command format. Use: stop <RUNNER>");
                return Err(AppStatus::RuntimeError);
            }
        }

        if command == "exit" {
            // Stop all runners before exiting
            for runner in [
                AL, Blob, EIP1559, EIP7702, Legacy, Random,
            ] {
                let _ = self.stop_runner(runner).await;
            }
            return Err(AppStatus::Exit);
        }

        if command == "start" {
            for runner in [
                AL, Blob, EIP1559, EIP7702, Legacy, Random,
            ] {
                if let Err(e) = self.start_runner(runner).await {
                    self.print(&format!("error starting runner: {}", e));
                    return Err(AppStatus::RuntimeError);
                }
            }
            self.print("all runners started");
            return Ok(());
        }

        if command.starts_with("start ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let runner = parts[1];
                if !self.is_valid_runner(runner) {
                    self.print(&format!("invalid runner: {}", runner));
                    return Err(AppStatus::RuntimeError);
                }

                if self.active_runners.contains_key(&Runner::from_str(runner).unwrap()) {
                    self.print(&format!("{} runner is already active", runner));
                    return Err(AppStatus::RuntimeError);
                }

                if let Err(e) = self.start_runner(Runner::from_str(runner).unwrap()).await {
                    self.print(&format!("error starting runner: {}", e));
                    return Err(AppStatus::RuntimeError);
                }
                self.print(&format!("{} runner started", runner));
                return Ok(());
            } else {
                self.print("invalid start command format. Use: start <RUNNER>");
                return Err(AppStatus::RuntimeError);
            }
        }

        self.print("invalid command");
        Err(AppStatus::RuntimeError)
    }
}
