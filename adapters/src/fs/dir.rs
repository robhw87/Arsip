use crate::fs::FsError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootDir(PathBuf);

impl RootDir {
    pub fn new(raw: impl Into<PathBuf>) -> Result<Self, FsError> {
        let path = raw.into();
        let meta = fs::metadata(&path).map_err(|source| match source.kind() {
            io::ErrorKind::NotFound => FsError::NotFound {
                path: path.clone(),
                source,
            },
            io::ErrorKind::PermissionDenied => FsError::PermissionDenied {
                path: path.clone(),
                source,
            },
            _ => FsError::IoError(source),
        })?;
        if !meta.is_dir() {
            return Err(FsError::NotADirectory { path });
        }
        fs::read_dir(&path).map_err(|source| FsError::ReadFailed {
            path: path.clone(),
            source,
        })?;
        Ok(RootDir(path))
    }
    pub fn as_path(&self) -> &Path {
        &self.0
    }
    pub fn scan_md(&self) -> Result<Vec<MarkdownPath>, FsError> {
        let mut result: Vec<MarkdownPath> = Vec::new();
        for entry in fs::read_dir(self.as_path())? {
            let fl = entry?.path();
            if fl.is_dir() {
                continue;
            }
            match fl.extension().and_then(|e| e.to_str()) {
                Some("md") | Some("markdown") => result.push(MarkdownPath::new(fl)),
                _ => continue,
            }
        }
        Ok(result)
    }
}

pub struct MarkdownPath(PathBuf);
impl MarkdownPath {
    pub fn new(path: PathBuf) -> Self {
        MarkdownPath(path)
    }
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::dir::RootDir;
    #[test]
    fn scan_dir_test() {
        let str_path = "/home/rnz23/projects/contents/";
        let result = RootDir::new(str_path);
        assert!(result.is_ok(), "Fail, result :  {:?}", result);
    }
}
