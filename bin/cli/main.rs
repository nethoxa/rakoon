use alloy::{hex, signers::k256::ecdsa::SigningKey};
use app::{App, config::Config};
use clap::Parser;

#[derive(Parser)]
#[command(name = "rakoon")]
#[command(about = "Transaction fuzzer for the Ethereum protocol")]
struct Cli {
    #[arg(long, help = "RPC URL to send transactions to", default_value = "http://localhost:8545")]
    rpc: String,
    #[arg(
        long,
        help = "Faucet key",
        default_value = "0xcdfbe6f7602f67a97602e3e9fc24cde1cdffa88acd47745c0b84c5ff55891e1b"
    )]
    sk: String,
    #[arg(long, help = "Path to the kurtosis network params file", default_value = "")]
    params: String,
    #[arg(long, help = "Seed for the random number generator", default_value = "0")]
    seed: u64,
    #[arg(long, help = "Enable random transaction fuzzing", default_value = "true")]
    random: bool,
    #[arg(long, help = "Enable legacy transaction fuzzing", default_value = "true")]
    legacy: bool,
    #[arg(long, help = "Enable access list transaction fuzzing", default_value = "true")]
    al: bool,
    #[arg(long, help = "Enable blob transaction fuzzing", default_value = "true")]
    blob: bool,
    #[arg(long, help = "Enable EIP-1559 transaction fuzzing", default_value = "true")]
    eip1559: bool,
    #[arg(long, help = "Enable EIP-7702 transaction fuzzing", default_value = "true")]
    eip7702: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let rpc_url = cli.rpc;
    let sk = SigningKey::from_slice(hex::decode(cli.sk).unwrap().as_slice()).unwrap();
    let seed = cli.seed;

    let config = Config {
        rpc_url,
        sk,
        seed,
        random_enabled: cli.random,
        legacy_enabled: cli.legacy,
        al_enabled: cli.al,
        blob_enabled: cli.blob,
        eip1559_enabled: cli.eip1559,
        eip7702_enabled: cli.eip7702,
    };

    let mut app = App::new(config);
    let _ = app.run().unwrap();
}
