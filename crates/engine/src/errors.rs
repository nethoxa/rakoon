use std::fmt::{Display, Formatter, Result};

pub enum EngineError {
    AlreadyRunning,
    AlreadyStopped,
    AlreadyPaused,
    ReturnedError(String),
    InvalidCommand(String),
    EngineNotRunning,
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EngineError::AlreadyRunning => write!(f, "Engine is already running"),
            EngineError::AlreadyStopped => write!(f, "Engine is already stopped"),
            EngineError::AlreadyPaused => write!(f, "Engine is already paused"),
            EngineError::ReturnedError(error) => write!(f, "Returned error: {}", error),
            EngineError::InvalidCommand(command) => write!(f, "Invalid command: {}", command),
            EngineError::EngineNotRunning => write!(f, "Engine is not running"),
        }
    }
}
