use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::Local;

/// Logger structure for writing logs to a file with timestamp
pub struct Logger {
    file: File,
    runner_name: String,
}

impl Logger {
    /// Creates a new logger instance for the specified runner
    ///
    /// # Arguments
    ///
    /// * `runner_name` - Name of the runner to be used in the log file name
    ///
    /// # Returns
    ///
    /// A Result containing the Logger instance or an IO error
    pub fn new(runner_name: &str) -> io::Result<Self> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("{}_log.log", runner_name);
        
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(Path::new(&filename))?;

        let startup_message = format!("[{}] Logger for {} started\n", timestamp, runner_name);
        file.write_all(startup_message.as_bytes())?;
        file.flush()?;
        
        Ok(Self {
            file,
            runner_name: runner_name.to_string(),
        })
    }
    
    /// Logs a message with timestamp
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    /// # Returns
    ///
    /// A Result indicating success or an IO error
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_line = format!("[{}] {}\n", timestamp, message);

        self.file.write_all(log_line.as_bytes())?;
        self.file.flush()
    }
    
    /// Logs an error message with timestamp
    ///
    /// # Arguments
    ///
    /// * `error` - The error message to log
    ///
    /// # Returns
    ///
    /// A Result indicating success or an IO error
    pub fn log_error(&mut self, error: &str) -> io::Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_line = format!("[{}] ERROR:{}\n", timestamp, error);

        self.file.write_all(log_line.as_bytes())?;
        self.file.flush()
    }
}
