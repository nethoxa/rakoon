use std::{
    fmt::{self, Display},
    str::FromStr,
};

pub mod al;
pub mod blob;
pub mod builder;
pub mod cache;
pub mod eip1559;
pub mod eip7702;
pub mod legacy;
pub mod logger;
pub mod random;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Runner {
    AL,
    Blob,
    EIP1559,
    EIP7702,
    Legacy,
    Random,
}

impl Display for Runner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Runner::AL => write!(f, "al"),
            Runner::Blob => write!(f, "blob"),
            Runner::EIP1559 => write!(f, "eip1559"),
            Runner::EIP7702 => write!(f, "eip7702"),
            Runner::Legacy => write!(f, "legacy"),
            Runner::Random => write!(f, "random"),
        }
    }
}

impl FromStr for Runner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "al" => Ok(Runner::AL),
            "blob" => Ok(Runner::Blob),
            "eip1559" => Ok(Runner::EIP1559),
            "eip7702" => Ok(Runner::EIP7702),
            "legacy" => Ok(Runner::Legacy),
            "random" => Ok(Runner::Random),
            _ => Err(format!("invalid runner: {}", s)),
        }
    }
}
