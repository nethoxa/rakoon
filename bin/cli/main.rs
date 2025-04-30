use app::App;
use clap::{App as ClapApp, Arg};

fn main() {
    let matches = ClapApp::new("Ethereum Fuzzer")
        .version("1.0")
        .author("nethoxa")
        .about("Transaction fuzzer for the Ethereum protocol")
        .arg(
            Arg::with_name("rpc")
                .long("rpc")
                .help("RPC URL to send transactions to (required)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("sk")
                .long("sk")
                .help("Faucet key (required)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .help("Seed for random generation")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .short("r")
                .help("Enable random transaction fuzzing")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("legacy")
                .long("legacy")
                .short("l")
                .help("Enable legacy transaction fuzzing")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("access-list")
                .long("access-list")
                .short("a")
                .help("Enable access list transaction fuzzing")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("blob")
                .long("blob")
                .short("b")
                .help("Enable blob transaction fuzzing")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("eip1559")
                .long("eip1559")
                .short("1559")
                .help("Enable EIP-1559 transaction fuzzing")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("eip7702")
                .long("eip7702")
                .short("7702")
                .help("Enable EIP-7702 transaction fuzzing")
                .takes_value(false),
        )
        .get_matches();

    // Create the engine first
    let mut engine = Engine::new(matches.value_of("rpc").unwrap().to_string());
    
    // Set master signing key
    let master_key = matches.value_of("master-key").unwrap();
    engine.set_master_key(master_key);
    
    // Set optional parameters
    if let Some(seed) = matches.value_of("seed") {
        if let Ok(seed_value) = seed.parse::<u64>() {
            engine.seed = seed_value;
        }
    }
    
    if let Some(corpus_path) = matches.value_of("corpus") {
        engine.load_corpus(corpus_path);
    }
    
    if let Some(ops) = matches.value_of("ops") {
        if let Ok(ops_value) = ops.parse::<u64>() {
            engine.max_operations_per_mutation = ops_value;
        }
    }
    
    // Configure fuzzer based on flags
    if matches.is_present("random") {
        engine.random_txs = true;
    }
    if matches.is_present("legacy") {
        engine.legacy_txs = true;
        engine.legacy_creation_txs = true;
    }
    if matches.is_present("access-list") {
        engine.empty_al_txs = true;
        engine.empty_al_creation_txs = true;
    }
    if matches.is_present("blob") {
        engine.blob_txs = true;
        engine.blob_creation_txs = true;
        engine.blob_al_txs = true;
        engine.blob_al_creation_txs = true;
    }
    if matches.is_present("eip1559") {
        engine.eip1559_txs = true;
        engine.eip1559_creation_txs = true;
        engine.eip1559_al_txs = true;
        engine.eip1559_al_creation_txs = true;
    }
    if matches.is_present("eip7702") {
        engine.auth_txs = true;
        engine.auth_creation_txs = true;
        engine.auth_al_txs = true;
        engine.auth_al_creation_txs = true;
        engine.auth_blob_txs = true;
        engine.auth_blob_creation_txs = true;
        engine.auth_blob_al_txs = true;
        engine.auth_blob_al_creation_txs = true;
    }
    if matches.is_present("mix") {
        // Enable all transaction types
        engine.random_txs = true;
        engine.legacy_txs = true;
        engine.legacy_creation_txs = true;
        engine.empty_al_txs = true;
        engine.empty_al_creation_txs = true;
        engine.eip1559_txs = true;
        engine.eip1559_creation_txs = true;
        engine.eip1559_al_txs = true;
        engine.eip1559_al_creation_txs = true;
        engine.blob_txs = true;
        engine.blob_creation_txs = true;
        engine.blob_al_txs = true;
        engine.blob_al_creation_txs = true;
        engine.auth_txs = true;
        engine.auth_creation_txs = true;
        engine.auth_al_txs = true;
        engine.auth_al_creation_txs = true;
        engine.auth_blob_txs = true;
        engine.auth_blob_creation_txs = true;
        engine.auth_blob_al_txs = true;
        engine.auth_blob_al_creation_txs = true;
    }

    // Create the app with the configured engine
    let mut app = App::new(engine);
    let _ = app.run();
}
