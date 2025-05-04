use std::{fmt::{self, Display}, str::FromStr};

pub mod al;
pub mod blob;
pub mod builder;
pub mod eip1559;
pub mod eip7702;
pub mod legacy;
pub mod random;

#[derive(PartialEq, Eq, Hash)]
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
            Runner::AL => write!(f, "AL"),
            Runner::Blob => write!(f, "Blob"),
            Runner::EIP1559 => write!(f, "EIP1559"),
            Runner::EIP7702 => write!(f, "EIP7702"),
            Runner::Legacy => write!(f, "Legacy"),
            Runner::Random => write!(f, "Random"),
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
