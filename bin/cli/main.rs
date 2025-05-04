use alloy::{hex, signers::k256::ecdsa::SigningKey, transports::http::reqwest::Url};
use app::App;
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
    #[arg(long, help = "Seed for the random number generator", default_value = "0")]
    seed: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let rpc_url = cli.rpc.parse::<Url>().unwrap();
    let sk = SigningKey::from_slice(hex::decode(cli.sk).unwrap().as_slice()).unwrap();
    let seed = cli.seed;

    let mut app = App::new(rpc_url, sk, seed);
    let _ = app.run().await.unwrap();
}
