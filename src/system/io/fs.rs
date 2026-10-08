use super::IoError;
use serde::Serialize;
use std::fs;
use std::path::Path;

pub fn read_md_file_to_string<P: AsRef<Path>>(filepath: P) -> Result<String, IoError> {
    let path = filepath.as_ref();
    let extension = path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("md") | Some("markdown") => fs::read_to_string(path).map_err(IoError::Io),
        _ => Err(IoError::NotMdFile), // Return custom error if not Markdown
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveOutcome {
    Renamed,
    CopiedAndDeleted,
    SourceNotFound,
}

pub async fn move_single_file(src: &Path, dst: &Path) -> Result<MoveOutcome, IoError> {
    if !tokio::fs::try_exists(src).await? {
        return Ok(MoveOutcome::SourceNotFound);
    }
    if let Some(parent) = dst.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    if tokio::fs::rename(src, dst).await.is_ok() {
        return Ok(MoveOutcome::Renamed);
    }
    tokio::fs::copy(src, dst).await?;
    tokio::fs::remove_file(src).await?;
    Ok(MoveOutcome::CopiedAndDeleted)
}
