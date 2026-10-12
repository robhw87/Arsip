use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Error : {0}")]
    GeneralError(String)
}
