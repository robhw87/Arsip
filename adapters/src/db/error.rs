use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Invalid Input")]
    InvalidInput,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("No Record To Insert")]
    EmptyRecords,
    #[error("Failed to build database connection pool: {0}")]
    DatabaseConnection(#[source] sqlx::Error),
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheck(#[source] sqlx::Error),
    #[error("Database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Pool Fail Created: {0}")]
    PoolFail(#[source] sqlx::Error),
}
