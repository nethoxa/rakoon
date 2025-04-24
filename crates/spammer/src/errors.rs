#[derive(Debug)]
pub enum SpammerError {
    FailedToConnect(String),
    FailedToReadCorpus(String),
    ProviderError(String),
    SigningError(String),
}
