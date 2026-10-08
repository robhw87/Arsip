use thiserror::Error;

#[derive(Debug, Error)]
pub enum MiddlewareError {
    #[error("Serialize failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("UNAUTHORIZED: {0}")]
    Unauthorized(String),
    #[error("No Authentication: {0}")]
    NoAuthToken(String),
    #[error("Login Fail: {0}")]
    LoginFail(String),
}
