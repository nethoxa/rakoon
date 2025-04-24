#[derive(Debug)]
/// SpammerError is an enum that contains the errors that can occur in the spammer.
pub enum SpammerError {
    /// Failed to connect to the provider.
    FailedToConnect(String),
    /// Failed to read the corpus.
    FailedToReadCorpus(String),
    /// Provider error.
    ProviderError(String),
    /// Signing error.
    SigningError(String),
}
