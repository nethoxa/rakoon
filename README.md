# tx-fuzz

This is a rewrite of the tx fuzzer written by [MariusVanDerWijden](https://github.com/MariusVanDerWijden) in Rust with enhanced performance. See the [benchmarks](#benchmarks) section below.

## Usage

First, build the project with:

```bash
cargo build --release
```

The supported commands are:

- `airdrop`: Sends some ETH to a pre-set list of accounts
    - `--sk`: Private key of the account to send ETH from
    - `--rpc`: RPC endpoint to connect to
- `spam`: Sends spam transactions
    - `--sk`: Private key of the account to send spam from
    - `--seed`: Seed for the random number generator
    - `--no-al`: Whether to use access list or not
    - `--corpus`: Path to the corpus file
    - `--rpc`: RPC endpoint to connect to
    - `--tx-count`: Number of transactions to send
    - `--gas-limit`: Gas limit for each transaction
    - `--slot-time`: Slot time in seconds
    - `--max-operations-per-mutation`: Maximum number of operations per mutation
- `blobs`: Sends blob spam transactions
    - `--sk`: Private key of the account to send spam from
    - `--seed`: Seed for the random number generator
    - `--no-al`: Whether to use access list or not
    - `--corpus`: Path to the corpus file
    - `--rpc`: RPC endpoint to connect to
    - `--tx-count`: Number of transactions to send
    - `--gas-limit`: Gas limit for each transaction
    - `--slot-time`: Slot time in seconds
    - `--max-operations-per-mutation`: Maximum number of operations per mutation
- `pectra`: Sends 7702 spam transactions
    - `--sk`: Private key of the account to send spam from
    - `--seed`: Seed for the random number generator
    - `--no-al`: Whether to use access list or not
    - `--corpus`: Path to the corpus file
    - `--rpc`: RPC endpoint to connect to
    - `--tx-count`: Number of transactions to send
    - `--gas-limit`: Gas limit for each transaction
    - `--slot-time`: Slot time in seconds
    - `--max-operations-per-mutation`: Maximum number of operations per mutation

## Benchmarks
TODO