use adapters::fs::FileContent;
use adapters::{AppClock, fs::RootDir};
use domain::content_validation::classify::ClassifyResult;
use domain::scan::report::ScanReport;
use time::Duration;
use time::Time::Instant;
pub mod error;
pub use error::ScanError;

pub struct ScanSession {
    pub root: RootDir,
    pub started_at: Timestamp,
    pub duration: Duration,
    pub files: ScanReport,
}

pub fn handle_scan(str_path: &str) -> Result<ScanSession, ScanError> {
    let root = RootDir::new(str_path)?;
    let clock = AppClock::new();
    let started_at = clock::timestamp();
    let mut elapse = Instant::now();
    let path_collections = root.scan_md()?;
    let mut records = Vec::new();
    for path in path_collections {
        let content = FileContent::read(path)?;
        records.push(classify(content.as_str())?);
    }
    let files = ScanReport ( records };
    let duration = elapse.elapsed();
    Ok(ScanSession {
        root,
        started_at,
        duration,
        files,
    })
}

pub struct ScanSummary {
    
}

pub enum ScanExitCode {
    Clean,      // 0
    Incomplete, // 1
    Fatal,      // 2
}
