use clap::{Parser, Subcommand};
use colored::Colorize;
use engine::Engine;

#[derive(Parser)]
#[command(name = "tx-fuzz")]
#[command(about = "Fuzzer for sending spam transactions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Sends some ETH to a pre-set list of accounts")]
    Airdrop {
        #[arg(long, help = "Private key of the account to send ETH from")]
        sk: String,
        #[arg(long, help = "RPC endpoint to connect to")]
        rpc: String,
    },
    #[command(about = "Sends spam transactions")]
    Spam {
        #[arg(long, help = "Private key of the account to send spam from")]
        sk: String,
        #[arg(long, help = "Seed for the random number generator")]
        seed: u64,
        #[arg(long, help = "Whether to use access list or not")]
        no_al: bool,
        #[arg(long, help = "Path to the corpus file")]
        corpus: String,
        #[arg(long, help = "RPC endpoint to connect to")]
        rpc: String,
        #[arg(long, help = "Number of transactions to send")]
        tx_count: u64,
        #[arg(long, help = "Gas limit for each transaction")]
        gas_limit: u64,
        #[arg(long, help = "Slot time in seconds")]
        slot_time: u64,
        #[arg(long, help = "Maximum number of operations per mutation")]
        max_operations_per_mutation: u64,
    },
    #[command(about = "Sends blob spam transactions")]
    Blobs {
        #[arg(long, help = "Private key of the account to send spam from")]
        sk: String,
        #[arg(long, help = "Seed for the random number generator")]
        seed: u64,
        #[arg(long, help = "Whether to use access list or not")]
        no_al: bool,
        #[arg(long, help = "Path to the corpus file")]
        corpus: String,
        #[arg(long, help = "RPC endpoint to connect to")]
        rpc: String,
        #[arg(long, help = "Number of transactions to send")]
        tx_count: u64,
        #[arg(long, help = "Gas limit for each transaction")]
        gas_limit: u64,
        #[arg(long, help = "Slot time in seconds")]
        slot_time: u64,
        #[arg(long, help = "Maximum number of operations per mutation")]
        max_operations_per_mutation: u64,
    },
    #[command(about = "Sends 7702 spam transactions")]
    Pectra {
        #[arg(long, help = "Private key of the account to send spam from")]
        sk: String,
        #[arg(long, help = "Seed for the random number generator")]
        seed: u64,
        #[arg(long, help = "Whether to use access list or not")]
        no_al: bool,
        #[arg(long, help = "Path to the corpus file")]
        corpus: String,
        #[arg(long, help = "RPC endpoint to connect to")]
        rpc: String,
        #[arg(long, help = "Number of transactions to send")]
        tx_count: u64,
        #[arg(long, help = "Gas limit for each transaction")]
        gas_limit: u64,
        #[arg(long, help = "Slot time in seconds")]
        slot_time: u64,
        #[arg(long, help = "Maximum number of operations per mutation")]
        max_operations_per_mutation: u64,
    },
}

#[tokio::main]
async fn main() {
    let mut engine = Engine::default();
    let cli = Cli::parse();

    match cli.command {
        Commands::Airdrop { sk, rpc } => {
            engine.set_sk(sk.clone());
            engine.set_rpc(rpc.clone());

            println!("[{}] Running airdrop with sk: {}, rpc: {}", "+".bright_green(), sk, rpc);

            match engine.run_airdrop().await {
                Ok(_) => println!("[{}] Airdrop completed successfully", "+".bright_green()),
                Err(e) => println!("[{}] Airdrop failed: {}", "-".bright_red(), e),
            }
        }
        Commands::Spam {
            sk,
            rpc,
            seed,
            no_al,
            corpus,
            tx_count,
            gas_limit,
            slot_time,
            max_operations_per_mutation,
        } => {
            engine.set_sk(sk.clone());
            engine.set_rpc(rpc.clone());
            engine.set_seed(seed);
            engine.set_no_al(no_al);
            engine.set_corpus(corpus);
            engine.set_tx_count(tx_count);
            engine.set_gas_limit(gas_limit);
            engine.set_slot_time(slot_time);
            engine.set_max_operations_per_mutation(max_operations_per_mutation);

            println!(
                "[{}] Running spam with sk: {}, rpc: {}, tx_count: {}, slot_time: {}",
                "+".bright_green(),
                sk,
                rpc,
                tx_count,
                slot_time
            );

            match engine.run_spam().await {
                Ok(_) => println!("[{}] Spam completed successfully", "+".bright_green()),
                Err(e) => println!("[{}] Spam failed: {}", "-".bright_red(), e),
            }
        }
        Commands::Blobs {
            sk,
            rpc,
            seed,
            no_al,
            corpus,
            tx_count,
            gas_limit,
            slot_time,
            max_operations_per_mutation,
        } => {
            engine.set_sk(sk.clone());
            engine.set_rpc(rpc.clone());
            engine.set_seed(seed);
            engine.set_no_al(no_al);
            engine.set_corpus(corpus);
            engine.set_tx_count(tx_count);
            engine.set_gas_limit(gas_limit);
            engine.set_slot_time(slot_time);
            engine.set_max_operations_per_mutation(max_operations_per_mutation);

            println!(
                "[{}] Running blob spam with sk: {}, rpc: {}, tx_count: {}, slot_time: {}",
                "+".bright_green(),
                sk,
                rpc,
                tx_count,
                slot_time
            );

            match engine.run_blob_spam() {
                Ok(_) => println!("[{}] Blob spam completed successfully", "+".bright_green()),
                Err(e) => println!("[{}] Blob spam failed: {}", "-".bright_red(), e),
            }
        }
        Commands::Pectra {
            sk,
            rpc,
            seed,
            no_al,
            corpus,
            tx_count,
            gas_limit,
            slot_time,
            max_operations_per_mutation,
        } => {
            engine.set_sk(sk.clone());
            engine.set_rpc(rpc.clone());
            engine.set_seed(seed);
            engine.set_no_al(no_al);
            engine.set_corpus(corpus);
            engine.set_tx_count(tx_count);
            engine.set_gas_limit(gas_limit);
            engine.set_slot_time(slot_time);
            engine.set_max_operations_per_mutation(max_operations_per_mutation);

            println!(
                "[{}] Running pectra spam with sk: {}, rpc: {}, tx_count: {}, slot_time: {}",
                "+".bright_green(),
                sk,
                rpc,
                tx_count,
                slot_time
            );

            match engine.run_7702_spam() {
                Ok(_) => println!("[{}] Pectra spam completed successfully", "+".bright_green()),
                Err(e) => println!("[{}] Pectra spam failed: {}", "-".bright_red(), e),
            }
        }
    }
}
