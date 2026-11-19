use adapters::fs::FileContent;
use adapters::{AppClock, fs::RootDir};
use domain::content_validation::classify::{ClassifyResult, classify};
use domain::scan::report::ScanReport;
use std::time::Instant;
use time::Duration;
use time::Timestamp;
pub mod error;
pub use error::ScanError;

pub struct ScanResult {
    pub root: RootDir,
    pub started_at: String,
    pub duration: Duration,
    pub files: ScanReport,
}

pub fn handle_scan(str_path: &str) -> Result<ScanResult, ScanError> {
    let root = RootDir::new(str_path)?;
    let clock = AppClock::new();
    let started_at = clock.timestamp();
    let mut elapse = Instant::now();
    let path_collections = root.scan_md()?;
    let mut records: Vec<ClassifyResult> = Vec::new();
    for path in path_collections {
        let content = FileContent::read(&path.as_path())?;
        records.push(classify(content.as_str())?);
    }
    let total_elapse = elapse.elapsed();
    let duration = Duration::new(
        total_elapse.as_secs() as i64,
        total_elapse.subsec_nanos() as i32,
    );
    Ok(ScanResult {
        root,
        started_at,
        duration,
        files,
    })
}
