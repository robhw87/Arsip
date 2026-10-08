pub mod clock;
pub mod error;
pub mod telemetry;
use crate::system::config::AppConfig;
use clock::AppClock;
use deadpool_redis::{Config, Connection, Manager, Pool, Runtime};
use error::StateError;
use redis::AsyncCommands;
use sqlx::ConnectOptions;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;
use tracing::log::LevelFilter;
use tracing::{error, info};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool, // One pool, shared everywhere
    pub config: AppConfig,
    pub clock: AppClock,
    pub redis_pool: Pool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, StateError> {
        info!("Initializing database...");
        Self::run_migrations(&config).await?;
        info!("Initializing connection pool...");
        let db_pool = Self::build_pool(&config).await?;
        Self::test_connection(&db_pool).await?;
        let clock = AppClock::new();
        let redis_pool = Self::redis_pool(&config).await?;
        Ok(Self {
            db_pool,
            config,
            clock,
            redis_pool,
        })
    }

    async fn build_pool(config: &AppConfig) -> Result<PgPool, StateError> {
        let mut connect_options: PgConnectOptions = config.database_url.parse()?;
        connect_options =
            connect_options.log_slow_statements(LevelFilter::Warn, Duration::from_secs(30));
        let pool = PgPoolOptions::new()
            .max_connections(config.db_max_conn)
            .acquire_timeout(Duration::from_secs(config.db_conn_timeout))
            .idle_timeout(Duration::from_secs(60))
            .connect_with(connect_options)
            .await
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                StateError::PoolFail(e)
            })?;
        Ok(pool)
    }

    async fn run_migrations(config: &AppConfig) -> Result<(), StateError> {
        info!("Starting Migration...");
        use sqlx::Connection;
        use sqlx::postgres::PgConnection;
        let mut conn = PgConnection::connect(&config.database_migrator_url)
            .await
            .map_err(StateError::Database)?;
        sqlx::migrate!("./migrations")
            .run(&mut conn)
            .await
            .map_err(StateError::Migration)?;
        info!("Finish Migration...");
        Ok(())
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }
    async fn test_connection(pool: &PgPool) -> Result<(), StateError> {
        sqlx::query("SELECT 1").fetch_one(pool).await.map_err(|e| {
            error!("Database health check failed: {}", e);
            StateError::DatabaseHealthCheck(e)
        })?;
        Ok(())
    }

    async fn redis_pool(config: &AppConfig) -> Result<Pool, StateError> {
        let cfg = Config::from_url(config.redis_url.clone());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        // println!("Redis Pool Status: {:?}", pool);
        Ok(pool)
    }
}
