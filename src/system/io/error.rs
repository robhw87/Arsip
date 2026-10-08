use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not MD Files, Skipped")]
    NotMdFile,
    #[error("Empty Data")]
    EmptyData,
    #[error("Struct And Header Column Count Mismatch")]
    CsvHeaderMismatch,
    #[error("Serialize failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Excel error: {0}")]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),
    #[error("Failed to create directory '{path}': {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to write file '{path}': {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
}
impl IoError {
    pub fn write_file(path: impl Into<PathBuf>) -> impl FnOnce(std::io::Error) -> Self {
        let path = path.into();
        move |source| IoError::WriteFile { path, source }
    }

    pub fn create_dir(path: impl Into<PathBuf>) -> impl FnOnce(std::io::Error) -> Self {
        let path = path.into();
        move |source| IoError::CreateDir { path, source }
    }
}
