use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    InvalidRpcUrl(String),
    RuntimeError,
    InvalidKey,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidRpcUrl(url) => write!(f, "invalid rpc url: {}", url),
            Error::RuntimeError => write!(f, "runtime error"),
            Error::InvalidKey => write!(f, "invalid key"),
        }
    }
}

