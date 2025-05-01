use app::App;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use al::ALTransactionRunner;
use blob::BlobTransactionRunner;
use eip1559::EIP1559TransactionRunner;
use eip7702::EIP7702TransactionRunner;
use legacy::LegacyTransactionRunner;
use random::RandomTransactionRunner;
use alloy::{hex, signers::k256::ecdsa::SigningKey};

fn main() {
    let mut app = App::new();
    app.run();
}

/*
#[derive(Parser)]
#[command(name = "rakoon")]
#[command(about = "Transaction fuzzer for the Ethereum protocol")]
struct Cli {
    #[arg(long, help = "RPC URL to send transactions to", required = true)]
    rpc: String,
    #[arg(long, help = "Faucet key", required = true)]
    sk: String,
    #[arg(long, help = "Path to the kurtosis network configuration file")]
    config: String,
    #[arg(long, help = "Seed for the random number generator")]
    seed: u64,
    #[arg(long, help = "Enable random transaction fuzzing")]
    random: bool,
    #[arg(long, help = "Enable legacy transaction fuzzing")]
    legacy: bool,
    #[arg(long, help = "Enable access list transaction fuzzing")]
    al: bool,
    #[arg(long, help = "Enable blob transaction fuzzing")]
    blob: bool,
    #[arg(long, help = "Enable EIP-1559 transaction fuzzing")]
    eip1559: bool,
    #[arg(long, help = "Enable EIP-7702 transaction fuzzing")]
    eip7702: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let rpc_url = cli.rpc;
    let sk = SigningKey::from_slice(hex::decode(cli.sk).unwrap().as_slice()).unwrap();
    let config = cli.config; // [nethoxa] TODO
    let seed = cli.seed;

    // Check if any transaction type is enabled
    let random_enabled = cli.random;
    let legacy_enabled = cli.legacy;
    let al_enabled = cli.al;
    let blob_enabled = cli.blob;
    let eip1559_enabled = cli.eip1559;
    let eip7702_enabled = cli.eip7702;

    let token = CancellationToken::new();
    let mut handles = vec![];
    
    if random_enabled {
        let runner = RandomTransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let random_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(random_token).await.unwrap();
        }));
    }

    if legacy_enabled {
        let runner = LegacyTransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let legacy_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(legacy_token).await.unwrap();
        }));
    }

    if al_enabled {
        let runner = ALTransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let al_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(al_token).await.unwrap();
        }));
    }

    if blob_enabled {
        let runner = BlobTransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let blob_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(blob_token).await.unwrap();
        }));
    }

    if eip1559_enabled {
        let runner = EIP1559TransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let eip1559_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(eip1559_token).await.unwrap();
        }));
    }

    if eip7702_enabled {
        let runner = EIP7702TransactionRunner::new(rpc_url.clone(), sk.clone(), seed);
        let eip7702_token = token.clone();
        handles.push(tokio::spawn(async move {
            runner.run(eip7702_token).await.unwrap();
        }));
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }
}
*/