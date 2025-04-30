#[derive(Debug)]
pub enum Error {
    InvalidRpcUrl(String),
    RuntimeError,
}
