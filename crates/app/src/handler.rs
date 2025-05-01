use crate::{errors::AppStatus, App};
use alloy::primitives::Address;
use crate::errors;

impl App {
    pub fn handle_command(&mut self, command: String) -> Result<(), AppStatus> {
        let command = command.trim();
        
        if command.starts_with("set ") {
            let parts: Vec<&str> = command.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let name = parts[1];
                let value = parts[2];
                
                match name {
                    "random" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.random_enabled = val;
                                *self.output.lock().unwrap() = format!("random fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for random: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "legacy" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.legacy_enabled = val;
                                *self.output.lock().unwrap() = format!("legacy fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for legacy: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "al" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.al_enabled = val;
                                *self.output.lock().unwrap() = format!("access list fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for al: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "blob" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.blob_enabled = val;
                                *self.output.lock().unwrap() = format!("blob fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for blob: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "eip1559" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.eip1559_enabled = val;
                                *self.output.lock().unwrap() = format!("eip1559 fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for eip1559: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "eip7702" => {
                        match value.parse::<bool>() {
                            Ok(val) => {
                                self.config.eip7702_enabled = val;
                                *self.output.lock().unwrap() = format!("eip7702 fuzzing set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid boolean value for eip7702: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "seed" => {
                        match value.parse::<u64>() {
                            Ok(val) => {
                                self.config.seed = val;
                                *self.output.lock().unwrap() = format!("seed set to {}", val);
                                return Ok(());
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid u64 value for seed: {}", value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    "rpc" => {
                        self.config.rpc_url = value.to_string();
                        *self.output.lock().unwrap() = format!("rpc url set to {}", value);
                        return Ok(());
                    },
                    "sk" => {
                        match alloy::hex::decode(value) {
                            Ok(decoded) => {
                                match alloy::signers::k256::ecdsa::SigningKey::from_slice(decoded.as_slice()) {
                                    Ok(key) => {
                                        self.config.sk = key;
                                        *self.output.lock().unwrap() = format!("signing key updated successfully");
                                        return Ok(());
                                    },
                                    Err(_) => {
                                        *self.output.lock().unwrap() = format!("invalid signing key format");
                                        return Err(AppStatus::RuntimeError);
                                    }
                                }
                            },
                            Err(_) => {
                                *self.output.lock().unwrap() = format!("invalid hex string for signing key");
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    },
                    _ => {
                        *self.output.lock().unwrap() = format!("unknown config parameter: {}", name);
                        return Err(AppStatus::RuntimeError);
                    }
                }
            } else {
                *self.output.lock().unwrap() = "invalid set command format. Use: set <NAME> <VALUE>".to_string();
                return Err(AppStatus::RuntimeError);
            }
        }
        
        if command.starts_with("get ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let name = parts[1];
                
                match name {
                    "random" => {
                        *self.output.lock().unwrap() = format!("random fuzzing: {}", self.config.random_enabled);
                        return Ok(());
                    },
                    "legacy" => {
                        *self.output.lock().unwrap() = format!("legacy fuzzing: {}", self.config.legacy_enabled);
                        return Ok(());
                    },
                    "al" => {
                        *self.output.lock().unwrap() = format!("access list fuzzing: {}", self.config.al_enabled);
                        return Ok(());
                    },
                    "blob" => {
                        *self.output.lock().unwrap() = format!("blob fuzzing: {}", self.config.blob_enabled);
                        return Ok(());
                    },
                    "eip1559" => {
                        *self.output.lock().unwrap() = format!("eip1559 fuzzing: {}", self.config.eip1559_enabled);
                        return Ok(());
                    },
                    "eip7702" => {
                        *self.output.lock().unwrap() = format!("eip7702 fuzzing: {}", self.config.eip7702_enabled);
                        return Ok(());
                    },
                    "seed" => {
                        *self.output.lock().unwrap() = format!("seed: {}", self.config.seed);
                        return Ok(());
                    },
                    "rpc" => {
                        *self.output.lock().unwrap() = format!("rpc url: {}", self.config.rpc_url);
                        return Ok(());
                    },
                    "sk" => {
                        let address = Address::from_private_key(&self.config.sk);
                        *self.output.lock().unwrap() = format!("signing key address: {}", address);
                        return Ok(());
                    },
                    "all" => {
                        let address = Address::from_private_key(&self.config.sk);

                        let mut output = String::new();
                        output.push_str(&format!("rpc url: {}\n", self.config.rpc_url));
                        output.push_str(&format!("signing key address: {}\n", address));
                        output.push_str(&format!("seed: {}\n", self.config.seed));
                        output.push_str(&format!("random fuzzing: {}\n", self.config.random_enabled));
                        output.push_str(&format!("legacy fuzzing: {}\n", self.config.legacy_enabled));
                        output.push_str(&format!("access list fuzzing: {}\n", self.config.al_enabled));
                        output.push_str(&format!("blob fuzzing: {}\n", self.config.blob_enabled));
                        output.push_str(&format!("eip1559 fuzzing: {}\n", self.config.eip1559_enabled));
                        output.push_str(&format!("eip7702 fuzzing: {}", self.config.eip7702_enabled));
                        *self.output.lock().unwrap() = output;
                        return Ok(());
                    },
                    _ => {
                        *self.output.lock().unwrap() = format!("unknown config parameter: {}", name);
                        return Err(AppStatus::RuntimeError);
                    }
                }
            } else {
                *self.output.lock().unwrap() = "invalid get command format. Use: get <NAME>".to_string();
                return Err(AppStatus::RuntimeError);
            }
        }
        
        match command {
            "start" => {
                self.status.clone_from(&"running".to_string());
                *self.output.lock().unwrap() = "fuzzers spawned correctly".to_string();
                Ok(())
            }
            "stop" => {
                self.status.clone_from(&"stopped".to_string());
                *self.output.lock().unwrap() = "fuzzers stopped correctly".to_string();
                Ok(())
            }
            "exit" => {
                Err(AppStatus::Exit)
            }
            _ => {
                *self.output.lock().unwrap() = "invalid command".to_string();
                Err(AppStatus::RuntimeError)
            }
        }
    }
}
