use crate::App;
use common::errors::Error;
use runners::{
    Runner, Runner::*, al::ALTransactionRunner, blob::BlobTransactionRunner,
    eip1559::EIP1559TransactionRunner, eip7702::EIP7702TransactionRunner,
    legacy::LegacyTransactionRunner, random::RandomTransactionRunner,
};

impl App {
    /// Starts a runner given its type. This function spawns a thread and
    /// stores its `handle` in the `handler` map. That way we can stop it
    /// later on by calling `abort` on the `handle`.
    ///
    /// # Arguments
    ///
    /// * `runner_type` - The type of the runner to start.
    pub async fn start_runner(&mut self, runner_type: Runner) -> Result<(), Error> {
        if *self.active_runners.get(&runner_type).unwrap_or(&false) {
            return Err(Error::RunnerAlreadyRunning);
        }

        let sk = self.runner_sks.get(&runner_type).unwrap_or(&self.sk).clone();
        let seed = *self.runner_seeds.get(&runner_type).unwrap_or(&self.seed);
        let rpc = self.runner_rpcs.get(&runner_type).unwrap_or(&self.rpc_url).clone();

        match runner_type {
            AL => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner =
                            ALTransactionRunner::new(rpc, sk, seed, max_operations_per_mutation);
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            Blob => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner =
                            BlobTransactionRunner::new(rpc, sk, seed, max_operations_per_mutation);
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            EIP1559 => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner = EIP1559TransactionRunner::new(
                            rpc,
                            sk,
                            seed,
                            max_operations_per_mutation,
                        );
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            EIP7702 => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner = EIP7702TransactionRunner::new(
                            rpc,
                            sk,
                            seed,
                            max_operations_per_mutation,
                        );
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            Legacy => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner = LegacyTransactionRunner::new(
                            rpc,
                            sk,
                            seed,
                            max_operations_per_mutation,
                        );
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            Random => {
                let handle = tokio::spawn({
                    let rpc = rpc.clone();
                    let sk = sk.clone();
                    let max_operations_per_mutation = self.max_operations_per_mutation;
                    async move {
                        let mut runner = RandomTransactionRunner::new(
                            rpc,
                            sk,
                            seed,
                            max_operations_per_mutation,
                        );
                        runner.run().await;
                    }
                });

                self.handler.insert(runner_type, handle);
            }
            _ => return Err(Error::RuntimeError),
        }

        self.active_runners.insert(runner_type, true);
        if self.active_runners.values().any(|&active| active) {
            self.running = true;
        }

        Ok(())
    }

    pub async fn stop_runner(&mut self, runner_type: Runner) -> Result<(), Error> {
        if !self.active_runners.get(&runner_type).unwrap_or(&false) {
            return Err(Error::RunnerAlreadyStopped);
        }

        // Get the handle for this runner and abort it
        if let Some(handle) = self.handler.remove(&runner_type) {
            // Cancel the handle to signal the runner to stop
            handle.abort();
        }

        // Remove the runner from the active runners map
        self.active_runners.remove(&runner_type);

        // If all runners are stopped, set the running flag to false
        if self.active_runners.values().all(|&active| !active) {
            self.running = false;
        }

        Ok(())
    }
}
