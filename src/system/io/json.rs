use crate::system::io::error::IoError;
use crate::system::state::clock;
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use tokio::fs;

pub async fn save_as_json<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), IoError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    if data.is_empty() {
        return Err(IoError::EmptyData);
    }
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir).await?;
    let file_path = dir.join(file_name);
    let created_at = clock::AppClock::new();
    let payload = json!({
        "expires_at": created_at.timestamp(),
        "data": data
    });
    let json_data = serde_json::to_string_pretty(&payload)?;
    fs::write(&file_path, json_data)
        .await
        .map_err(IoError::write_file(&file_path))?;
    Ok(())
}
