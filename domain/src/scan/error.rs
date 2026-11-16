use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Error : {0}")]
    GeneralError(String),
}
