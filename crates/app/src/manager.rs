use crate::App;
use common::errors::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

impl App {
    pub async fn start_runner(&mut self, runner_type: &str) -> Result<(), Error> {
        let rpc_url = self.config.rpc_url.clone();
        let sk = self.config.get_runner_sk(runner_type).clone();
        let seed = self.config.get_runner_seed(runner_type);
        let tx_counts = self.tx_counts.clone();

        match runner_type {
            "random" => {
                let runner = random::RandomTransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("random".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            "legacy" => {
                let runner = legacy::LegacyTransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("legacy".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            "al" => {
                let runner = al::ALTransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("al".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            "blob" => {
                let runner = blob::BlobTransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("blob".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            "eip1559" => {
                let runner = eip1559::EIP1559TransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("eip1559".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            "eip7702" => {
                let runner = eip7702::EIP7702TransactionRunner::new(rpc_url, sk, seed);
                let tx_counts = tx_counts.clone();
                let runner_clone = Arc::new(Mutex::new(runner));
                let handle = tokio::spawn({
                    let runner = runner_clone.clone();
                    async move {
                        let mut runner = runner.lock().await;
                        runner.run().await;
                    }
                });

                let mut handlers = self.handler.lock().unwrap();
                handlers.insert(runner_type.to_string(), handle);

                // [nethoxa] TODO does not work
                tokio::spawn({
                    let runner = runner_clone.clone();
                    let tx_counts = tx_counts.clone();
                    async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            let runner = runner.lock().await;
                            let mut counts = tx_counts.lock().unwrap();
                            counts.insert("eip7702".to_string(), runner.tx_sent);
                        }
                    }
                });
            }
            _ => return Err(Error::RuntimeError),
        }

        self.config.start_runner(runner_type.to_string());
        self.status = "running".to_string();
        Ok(())
    }

    pub async fn stop_runner(&mut self, runner_type: &str) -> Result<(), Error> {
        if !self.config.is_runner_active(runner_type) {
            return Err(Error::RuntimeError);
        }

        // Get the cancellation token for this runner and cancel it
        let mut handlers = self.handler.lock().unwrap();
        if let Some(handle) = handlers.remove(runner_type) {
            // Cancel the token to signal the runner to stop
            handle.abort();

            // Wait a short time for the runner to clean up
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        self.config.stop_runner(runner_type);
        if self.config.active_runners.is_empty() {
            self.status = "stopped".to_string();
        }
        Ok(())
    }
}
