use crate::system::io::error::IoError;
use rust_xlsxwriter::Workbook;
use serde::Serialize;
use std::path::Path;
use tokio::fs;

pub async fn save_as_xlsx<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), IoError>
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
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.serialize(&data)?;
    let excel_bytes = workbook.save_to_buffer()?;
    fs::write(&file_path, excel_bytes)
        .await
        .map_err(IoError::write_file(&file_path))?;
    Ok(())
}
