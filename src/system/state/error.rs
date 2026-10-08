use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Failed to build database connection pool: {0}")]
    DatabaseConnection(#[source] sqlx::Error),
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheck(#[source] sqlx::Error),
    #[error("Database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Pool Fail Created: {0}")]
    PoolFail(#[source] sqlx::Error),
    #[error("Time formatting error: {0}")]
    TimeFormat(#[from] time::error::Format),
    #[error("Time parsing error: {0}")]
    ParseStrTime(#[from] time::error::Parse),
    #[error("Redis Build error: {0}")]
    RedisBuildError(#[from] deadpool_redis::BuildError),
    #[error("Redis Config error: {0}")]
    RedisConfigError(#[from] deadpool_redis::ConfigError),
    #[error("Redis Config error: {0}")]
    RedisPoolError(#[from] deadpool_redis::CreatePoolError),
}
