use super::error::FsError;
use std::fs;
use std::path::Path;

pub struct FileContent(String);

impl FileContent {
    pub fn read(path: &Path) -> Result<Self, FsError> {
        let content = fs::read_to_string(path).map_err(|source| FsError::ReadFailed {
            path: path.to_owned(),
            source,
        })?;
        Ok(FileContent(content))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
