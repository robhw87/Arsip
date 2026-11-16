use crate::{cache, clock, config, db, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Error : {0}")]
    GeneralError(String),
    #[error("File I/O error: {0}")]
    FsError(#[from] fs::FsError),
    #[error("Database Processing Error: {0}")]
    DbError(#[from] db::DbError),
    #[error("Caching Processing Error: {0}")]
    CacheError(#[from] cache::CacheError),
    #[error("App Clock Error: {0}")]
    ClockError(#[from] clock::ClockError),
    #[error("App Config Error: {0}")]
    ConfigError(#[from] config::ConfigError),
}
