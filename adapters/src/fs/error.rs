use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Path not found: {path}")]
    NotFound {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Permission denied: {path}")]
    PermissionDenied {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Not a directory: {path}")]
    NotADirectory { path: PathBuf },
    #[error("Failed to read directory {path}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
