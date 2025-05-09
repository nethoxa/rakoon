use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};
use alloy::transports::{RpcError, TransportErrorKind};
use chrono::Local;

/// Logger structure for writing logs to a file with timestamp
pub struct Logger {
    file: File,
    runner_name: String,
}

impl Logger {
    fn ensure_directories(runner_name: &str) -> io::Result<()> {
        // Create logs directory if it doesn't exist
        std::fs::create_dir_all("logs")?;
        
        // Create reports directory and runner subdirectory if they don't exist
        let reports_dir = format!("reports/{}", runner_name);
        std::fs::create_dir_all(&reports_dir)?;
        
        Ok(())
    }

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
        // Ensure directories exist
        Self::ensure_directories(runner_name)?;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("logs/{}_log.log", runner_name);
        
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
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("[{}] {}\n", timestamp, message);
        self.file.write_all(log_message.as_bytes())?;
        self.file.flush()?;
        Ok(())
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

    pub fn generate_crash_report(&mut self, crash_data: &[u8]) -> io::Result<()> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let report_filename = format!("reports/{}/crash_report_{}.txt", self.runner_name, timestamp);
        
        let mut report_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Path::new(&report_filename))?;

        let formatted_data = format!("Transaction that crashed the node (hex): 0x{}\n", hex::encode(crash_data));
        report_file.write_all(formatted_data.as_bytes())?;

        report_file.flush()?;
        Ok(())
    }

    pub fn is_connection_refused_error(&self, err: &RpcError<TransportErrorKind>) -> bool {
        let formatted_err = format!("{:#?}", err);
        formatted_err.contains("Connection refused")
    }
}
