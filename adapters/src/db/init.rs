use super::DbError;
use crate::AppConfig;
use sqlx::ConnectOptions;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;
use tracing::log::LevelFilter;
use tracing::{error, info};

#[derive(Clone, Debug)]
pub struct AppDbPool(PgPool);

impl AppDbPool {
    pub async fn new(config: &AppConfig) -> Result<Self, DbError> {
        info!("Initializing connection pool...");
        let db_pool = Self::build_pool(config).await?;
        Self::test_connection(&db_pool).await?;
        Ok(Self(db_pool))
    }

    async fn build_pool(config: &AppConfig) -> Result<PgPool, DbError> {
        let mut connect_options: PgConnectOptions = config.database_url.parse()?;
        connect_options =
            connect_options.log_slow_statements(LevelFilter::Warn, Duration::from_secs(30));
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(15))
            .idle_timeout(Duration::from_secs(60))
            .connect_with(connect_options)
            .await
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                DbError::PoolFail(e)
            })?;
        Ok(pool)
    }

    pub fn pool(&self) -> &PgPool {
        &self.0
    }
    async fn test_connection(pool: &PgPool) -> Result<(), DbError> {
        sqlx::query("SELECT 1").fetch_one(pool).await.map_err(|e| {
            error!("Database health check failed: {}", e);
            DbError::DatabaseHealthCheck(e)
        })?;
        Ok(())
    }
}
