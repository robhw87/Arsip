use crate::system::cache::error::CacheError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Invalid Input")]
    InvalidInput,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("No Record To Insert")]
    EmptyRecords,
}
