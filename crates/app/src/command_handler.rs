use crate::{App, errors::AppError};
use alloy::{primitives::U256, rpc::types::TransactionRequest, signers::k256::ecdsa::SigningKey};
use colored::Colorize;

pub trait CommandHandler {
    fn handle_command_not_running(
        &mut self,
        command: &str,
        history: &mut Vec<(String, String)>,
    ) -> Result<(), AppError>;
    fn handle_command_running(
        &mut self,
        command: &str,
        history: &mut Vec<(String, String)>,
    ) -> Result<(), AppError>;
}

impl CommandHandler for App {
    fn handle_command_not_running(
        &mut self,
        command: &str,
        history: &mut Vec<(String, String)>,
    ) -> Result<(), AppError> {
        if command.trim().starts_with("set ") {
            let parts: Vec<&str> = command.trim().splitn(3, ' ').collect();
            if parts.len() < 3 {
                history.push((
                    command.to_string(),
                    format!(
                        "[{}] Invalid set command format. Use: set field value",
                        "-".bright_red()
                    ),
                ));
                return Ok(());
            }

            let field = parts[1];
            let value = parts[2];

            match field {
                "random_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_random_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set random_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "legacy_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_legacy_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set legacy_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "legacy_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_legacy_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set legacy_creation_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "empty_al_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_empty_al_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set empty_al_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "empty_al_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_empty_al_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set empty_al_creation_txs to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_creation_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_al_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_al_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_al_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_al_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_al_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set eip1559_al_creation_txs to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "blob_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_blob_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set blob_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "max_operations_per_mutation" => {
                    if let Ok(val) = value.parse::<u64>() {
                        self.engine.set_max_operations_per_mutation(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set max_operations_per_mutation to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid u64 value", "-".bright_red()),
                        ));
                    }
                }
                "max_input_length" => {
                    if let Ok(val) = value.parse::<usize>() {
                        self.engine.set_max_input_length(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set max_input_length to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid usize value", "-".bright_red()),
                        ));
                    }
                }
                "max_access_list_length" => {
                    if let Ok(val) = value.parse::<usize>() {
                        self.engine.set_max_access_list_length(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set max_access_list_length to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid usize value", "-".bright_red()),
                        ));
                    }
                }
                "max_transaction_type" => {
                    if let Ok(val) = value.parse::<u8>() {
                        self.engine.set_max_transaction_type(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set max_transaction_type to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid u8 value", "-".bright_red()),
                        ));
                    }
                }
                _ => {
                    history.push((
                        command.to_string(),
                        format!("[{}] Unknown field: {}", "-".bright_red(), field),
                    ));
                }
            }
            return Ok(());
        }

        match command.trim() {
            "start" => match self.engine.start() {
                Ok(output) => {
                    history.push((
                        command.to_string(),
                        format!("[{}] {}", "+".bright_green(), output),
                    ));
                }
                Err(e) => {
                    history.push((command.to_string(), format!("[{}] {}", "-".bright_red(), e)));
                }
            },
            "exit" => {
                return Err(AppError::AppExit);
            }
            _ => {
                history.push((
                    command.to_string(),
                    format!(
                        "[{}] {}",
                        "-".bright_red(),
                        AppError::InvalidCommand(command.to_string())
                    ),
                ));
            }
        }

        Ok(())
    }

    fn handle_command_running(
        &mut self,
        command: &str,
        history: &mut Vec<(String, String)>,
    ) -> Result<(), AppError> {
        if command.trim().starts_with("set ") {
            let parts: Vec<&str> = command.trim().splitn(3, ' ').collect();
            if parts.len() < 3 {
                history.push((
                    command.to_string(),
                    format!(
                        "[{}] Invalid set command format. Use: set field value",
                        "-".bright_red()
                    ),
                ));
                return Ok(());
            }

            let field = parts[1];
            let value = parts[2];

            match field {
                "random_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_random_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set random_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "legacy_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_legacy_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set legacy_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "legacy_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_legacy_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set legacy_creation_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "empty_al_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_empty_al_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set empty_al_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "empty_al_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_empty_al_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set empty_al_creation_txs to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_creation_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_al_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_al_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set eip1559_al_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "eip1559_al_creation_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_eip1559_al_creation_txs(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set eip1559_al_creation_txs to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "blob_txs" => {
                    if let Ok(val) = value.parse::<bool>() {
                        self.engine.set_blob_txs(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set blob_txs to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid boolean value", "-".bright_red()),
                        ));
                    }
                }
                "max_operations_per_mutation" => {
                    if let Ok(val) = value.parse::<u64>() {
                        self.engine.set_max_operations_per_mutation(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set max_operations_per_mutation to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid u64 value", "-".bright_red()),
                        ));
                    }
                }
                "max_input_length" => {
                    if let Ok(val) = value.parse::<usize>() {
                        self.engine.set_max_input_length(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set max_input_length to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid usize value", "-".bright_red()),
                        ));
                    }
                }
                "max_access_list_length" => {
                    if let Ok(val) = value.parse::<usize>() {
                        self.engine.set_max_access_list_length(val);
                        history.push((
                            command.to_string(),
                            format!(
                                "[{}] Set max_access_list_length to {}",
                                "+".bright_green(),
                                val
                            ),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid usize value", "-".bright_red()),
                        ));
                    }
                }
                "max_transaction_type" => {
                    if let Ok(val) = value.parse::<u8>() {
                        self.engine.set_max_transaction_type(val);
                        history.push((
                            command.to_string(),
                            format!("[{}] Set max_transaction_type to {}", "+".bright_green(), val),
                        ));
                    } else {
                        history.push((
                            command.to_string(),
                            format!("[{}] Invalid u8 value", "-".bright_red()),
                        ));
                    }
                }
                _ => {
                    history.push((
                        command.to_string(),
                        format!("[{}] Unknown field: {}", "-".bright_red(), field),
                    ));
                }
            }
            return Ok(());
        }

        match command.trim() {
            "start" => {
                history.push((
                    command.to_string(),
                    format!("[{}] {}", "-".bright_red(), "Engine already running"),
                ));
            }
            "stop" => match self.engine.stop() {
                Ok(output) => {
                    history.push((
                        command.to_string(),
                        format!("[{}] {}", "+".bright_green(), output),
                    ));
                }
                Err(e) => {
                    history.push((command.to_string(), format!("[{}] {}", "-".bright_red(), e)));
                }
            },
            "exit" => {
                return Err(AppError::AppExit);
            }
            _ => {
                history.push((
                    command.to_string(),
                    format!(
                        "[{}] {}",
                        "-".bright_red(),
                        AppError::InvalidCommand(command.to_string())
                    ),
                ));
            }
        }

        Ok(())
    }
}
