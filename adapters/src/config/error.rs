use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration Fail: {0}")]
    Config(String),
    #[error("Dotenv loading failed: {0}")]
    Dotenv(#[from] dotenvy::Error),
    #[error("Configuration Env error: {0}")]
    Envy(#[from] envy::Error),
}
