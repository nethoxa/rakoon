use crate::{App, errors::AppStatus};
use alloy::primitives::Address;

impl App {
    pub async fn handle_command(&mut self, command: String) -> Result<(), AppStatus> {
        let command = command.trim();

        if command.starts_with("set ") {
            let parts: Vec<&str> = command.splitn(4, ' ').collect();
            if parts.len() >= 3 {
                let scope = parts[1]; // "global" or transaction type
                let param = parts[2]; // "seed" or "sk"
                let value = parts.get(3).unwrap_or(&"");

                match (scope, param) {
                    ("global", "seed") => match value.parse::<u64>() {
                        Ok(val) => {
                            self.config.set_global_seed(val);
                            *self.output.lock().unwrap() = format!("global seed set to {}", val);
                            return Ok(());
                        }
                        Err(_) => {
                            *self.output.lock().unwrap() =
                                format!("invalid u64 value for global seed: {}", value);
                            return Err(AppStatus::RuntimeError);
                        }
                    },
                    ("global", "sk") => match alloy::hex::decode(value) {
                        Ok(decoded) => {
                            // [nethoxa] check how to make this work
                            match alloy::signers::k256::ecdsa::SigningKey::from_slice(
                                decoded.as_slice(),
                            ) {
                                Ok(key) => {
                                    self.config.set_global_sk(key);
                                    *self.output.lock().unwrap() =
                                        format!("global signing key updated successfully");
                                    return Ok(());
                                }
                                Err(_) => {
                                    *self.output.lock().unwrap() =
                                        format!("invalid signing key format");
                                    return Err(AppStatus::RuntimeError);
                                }
                            }
                        }
                        Err(_) => {
                            *self.output.lock().unwrap() =
                                format!("invalid hex string for signing key");
                            return Err(AppStatus::RuntimeError);
                        }
                    },
                    (tx_type, "seed")
                        if [
                            "random", "legacy", "al", "blob", "eip1559", "eip7702",
                        ]
                        .contains(&tx_type) =>
                    {
                        match value.parse::<u64>() {
                            Ok(val) => {
                                self.config.set_runner_seed(tx_type.to_string(), val);
                                *self.output.lock().unwrap() =
                                    format!("{} runner seed set to {}", tx_type, val);
                                return Ok(());
                            }
                            Err(_) => {
                                *self.output.lock().unwrap() =
                                    format!("invalid u64 value for {} seed: {}", tx_type, value);
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    }
                    (tx_type, "sk")
                        if [
                            "random", "legacy", "al", "blob", "eip1559", "eip7702",
                        ]
                        .contains(&tx_type) =>
                    {
                        match alloy::hex::decode(value) {
                            Ok(decoded) => {
                                // [nethoxa] check how to make this work
                                match alloy::signers::k256::ecdsa::SigningKey::from_slice(
                                    decoded.as_slice(),
                                ) {
                                    Ok(key) => {
                                        self.config.set_runner_sk(tx_type.to_string(), key);
                                        *self.output.lock().unwrap() = format!(
                                            "{} runner signing key updated successfully",
                                            tx_type
                                        );
                                        return Ok(());
                                    }
                                    Err(_) => {
                                        *self.output.lock().unwrap() =
                                            format!("invalid signing key format");
                                        return Err(AppStatus::RuntimeError);
                                    }
                                }
                            }
                            Err(_) => {
                                *self.output.lock().unwrap() =
                                    format!("invalid hex string for signing key");
                                return Err(AppStatus::RuntimeError);
                            }
                        }
                    }
                    _ => {
                        *self.output.lock().unwrap() = format!(
                            "invalid set command format. Use: set <global/TX_TYPE> <seed/sk> <VALUE>"
                        );
                        return Err(AppStatus::RuntimeError);
                    }
                }
            } else {
                *self.output.lock().unwrap() =
                    "invalid set command format. Use: set <global/TX_TYPE> <seed/sk> <VALUE>"
                        .to_string();
                return Err(AppStatus::RuntimeError);
            }
        }

        if command.starts_with("run ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let tx_type = parts[1];
                if ![
                    "random", "legacy", "al", "blob", "eip1559", "eip7702",
                ]
                .contains(&tx_type)
                {
                    *self.output.lock().unwrap() = format!("invalid transaction type: {}", tx_type);
                    return Err(AppStatus::RuntimeError);
                }

                if self.config.is_runner_active(tx_type) {
                    *self.output.lock().unwrap() = format!("{} runner is already active", tx_type);
                    return Err(AppStatus::RuntimeError);
                }

                self.start_runner(tx_type).await;
                *self.output.lock().unwrap() = format!("{} runner started", tx_type);
                return Ok(());
            } else {
                *self.output.lock().unwrap() =
                    "invalid run command format. Use: run <TX_TYPE>".to_string();
                return Err(AppStatus::RuntimeError);
            }
        }

        if command == "stop" {
            for runner in [
                "random", "legacy", "al", "blob", "eip1559", "eip7702",
            ] {
                self.stop_runner(runner).await;
            }
            *self.output.lock().unwrap() = "all runners stopped".to_string();
            return Ok(());
        }

        if command.starts_with("stop ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let tx_type = parts[1];
                if ![
                    "random", "legacy", "al", "blob", "eip1559", "eip7702",
                ]
                .contains(&tx_type)
                {
                    *self.output.lock().unwrap() = format!("invalid transaction type: {}", tx_type);
                    return Err(AppStatus::RuntimeError);
                }

                if !self.config.is_runner_active(tx_type) {
                    *self.output.lock().unwrap() = format!("{} runner is not active", tx_type);
                    return Err(AppStatus::RuntimeError);
                }

                self.stop_runner(tx_type).await;
                *self.output.lock().unwrap() = format!("{} runner stopped", tx_type);
                return Ok(());
            } else {
                *self.output.lock().unwrap() =
                    "invalid stop command format. Use: stop <TX_TYPE>".to_string();
                return Err(AppStatus::RuntimeError);
            }
        }

        if command.starts_with("get ") {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let name = parts[1];

                match name {
                    "random" => {
                        *self.output.lock().unwrap() =
                            format!("random fuzzing: {}", self.config.random_enabled);
                        return Ok(());
                    }
                    "legacy" => {
                        *self.output.lock().unwrap() =
                            format!("legacy fuzzing: {}", self.config.legacy_enabled);
                        return Ok(());
                    }
                    "al" => {
                        *self.output.lock().unwrap() =
                            format!("access list fuzzing: {}", self.config.al_enabled);
                        return Ok(());
                    }
                    "blob" => {
                        *self.output.lock().unwrap() =
                            format!("blob fuzzing: {}", self.config.blob_enabled);
                        return Ok(());
                    }
                    "eip1559" => {
                        *self.output.lock().unwrap() =
                            format!("eip1559 fuzzing: {}", self.config.eip1559_enabled);
                        return Ok(());
                    }
                    "eip7702" => {
                        *self.output.lock().unwrap() =
                            format!("eip7702 fuzzing: {}", self.config.eip7702_enabled);
                        return Ok(());
                    }
                    "seed" => {
                        *self.output.lock().unwrap() = format!("seed: {}", self.config.seed);
                        return Ok(());
                    }
                    "rpc" => {
                        *self.output.lock().unwrap() = format!("rpc url: {}", self.config.rpc_url);
                        return Ok(());
                    }
                    "sk" => {
                        let address = Address::from_private_key(&self.config.sk);
                        *self.output.lock().unwrap() = format!("signing key address: {}", address);
                        return Ok(());
                    }
                    "all" => {
                        let address = Address::from_private_key(&self.config.sk);

                        // [nethoxa] the \n does not work
                        let output_parts = vec![
                            format!("rpc url: {}", self.config.rpc_url),
                            format!("signing key address: {}", address),
                            format!("seed: {}", self.config.seed),
                            format!("random fuzzing: {}", self.config.random_enabled),
                            format!("legacy fuzzing: {}", self.config.legacy_enabled),
                            format!("access list fuzzing: {}", self.config.al_enabled),
                            format!("blob fuzzing: {}", self.config.blob_enabled),
                            format!("eip1559 fuzzing: {}", self.config.eip1559_enabled),
                            format!("eip7702 fuzzing: {}", self.config.eip7702_enabled),
                        ];

                        *self.output.lock().unwrap() = output_parts.join("\n");
                        return Ok(());
                    }
                    _ => {
                        *self.output.lock().unwrap() =
                            format!("unknown config parameter: {}", name);
                        return Err(AppStatus::RuntimeError);
                    }
                }
            } else {
                *self.output.lock().unwrap() =
                    "invalid get command format. Use: get <NAME>".to_string();
                return Err(AppStatus::RuntimeError);
            }
        }

        if command == "exit" {
            return Err(AppStatus::Exit);
        }

        *self.output.lock().unwrap() = "invalid command".to_string();
        Err(AppStatus::RuntimeError)
    }
}
