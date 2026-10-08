use crate::system::io::error::IoError;
use serde::Serialize;
use std::path::Path;
use tokio::fs;

pub async fn save_as_csv<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), IoError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    if data.is_empty() {
        return Err(IoError::EmptyData);
    }
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir)
        .await
        .map_err(IoError::create_dir(dir))?;
    let file_path = dir.join(file_name);
    let mut writer = csv::Writer::from_writer(Vec::new());
    for record in data {
        writer.serialize(record)?; // Headers written automatically on first row
    }
    let csv_bytes = writer.into_inner().map_err(|e| e.into_error())?;
    fs::write(&file_path, csv_bytes)
        .await
        .map_err(IoError::write_file(&file_path))?;
    Ok(())
}
