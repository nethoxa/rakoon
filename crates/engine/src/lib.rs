pub mod errors;

use crate::errors::EngineError;

#[derive(PartialEq, Eq)]
pub enum EngineStatus {
    Stopped,
    Running,
    Paused,
}

pub struct Engine {
    status: EngineStatus,
}

impl Engine {
    pub fn new() -> Self {
        Self { status: EngineStatus::Stopped }
    }

    pub fn is_running(&self) -> bool {
        self.status == EngineStatus::Running
    }

    pub fn is_paused(&self) -> bool {
        self.status == EngineStatus::Paused
    }

    pub fn is_stopped(&self) -> bool {
        self.status == EngineStatus::Stopped
    }

    pub fn start(&mut self) -> Result<String, EngineError> {
        if self.is_running() {
            return Err(EngineError::AlreadyRunning);
        }
        self.status = EngineStatus::Running;
        Ok("Engine started".to_string())
    }

    pub fn stop(&mut self) -> Result<String, EngineError> {
        if self.is_stopped() {
            return Err(EngineError::AlreadyStopped);
        }
        self.status = EngineStatus::Stopped;
        Ok("Engine stopped".to_string())
    }

    pub fn pause(&mut self) -> Result<String, EngineError> {
        if self.is_paused() {
            return Err(EngineError::AlreadyPaused);
        }
        self.status = EngineStatus::Paused;
        Ok("Engine paused".to_string())
    }
}
