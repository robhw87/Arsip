use adapters::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error : {0}")]
    GeneralError(String),
    #[error("File Read error: {0}")]
    FsError(#[from] fs::FsError),
}
