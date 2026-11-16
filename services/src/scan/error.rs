use adapters::error::AdapterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Error : {0}")]
    GeneralError(String),
    #[error("File Read error: {0}")]
    FsError(#[from] AdapterError),
}
