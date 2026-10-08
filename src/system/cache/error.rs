use crate::system;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    // -- IO and Parsing Errors
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Integer parsing failed: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Time parsing error: {0}")]
    ParseStrTime(#[from] time::error::Parse),
    #[error("Serialize failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Excel error: {0}")]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] system::db::error::DbError),
    // -- Execution and Resource Errors
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Cache data not available or invalid at path: {0:?}")]
    CacheMiss(PathBuf),
    #[error("Redis Error: {0}")]
    CacheRedisError(#[from] redis::RedisError),
    #[error("Unavailable Redis: {0}")]
    Unavailable(#[source] redis::RedisError),
    #[error("Redis Execution fail: {0}")]
    RedisExecutionFailed(#[source] redis::RedisError),
    #[error("No Cache in Redis")]
    CacheNotFound,
    #[error("Redis Pool Error")]
    PoolError(#[from] deadpool_redis::PoolError),
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}
