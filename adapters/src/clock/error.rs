use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClockError {
    #[error("Time formatting error: {0}")]
    TimeFormat(#[from] time::error::Format),
    #[error("Time parsing error: {0}")]
    ParseStrTime(#[from] time::error::Parse),
}
