use super::error::CacheError;
use crate::AppConfig;
use deadpool_redis::{Config, Pool, Runtime};
use redis::AsyncCommands;
#[derive(Debug, Clone)]

pub struct CacheEngine(Pool);

impl CacheEngine {
    async fn new(config: &AppConfig) -> Result<Self, CacheError> {
        let cfg = Config::from_url(config.redis_url.clone());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self(pool))
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, CacheError> {
        let mut conn = self.0.get().await.map_err(CacheError::CachePoolError)?;
        let value: Option<Vec<u8>> = conn
            .get(key)
            .await
            .map_err(CacheError::RedisExecutionFailed)?;

        // 3. Unpack value or return CacheNotFound error
        value.ok_or(CacheError::CacheNotFound)
    }

    async fn set(&self, key: &str, value: &[u8], ttl: u64) -> Result<(), CacheError> {
        let mut conn = self.0.get().await.map_err(CacheError::CachePoolError)?;
        let _: () = conn
            .set_ex(key, value, ttl)
            .await
            .map_err(CacheError::RedisExecutionFailed)?;

        Ok(())
    }
    pub async fn set_permanent(&self, key: &str, value: &[u8]) -> Result<bool, CacheError> {
        let mut conn = self.0.get().await.map_err(CacheError::CachePoolError)?;
        let _: () = conn
            .set(key, value)
            .await
            .map_err(CacheError::RedisExecutionFailed)?;
        Ok(true)
    }
    pub async fn del(&self, key: &str) -> Result<(), CacheError> {
        let mut conn = self.0.get().await.map_err(CacheError::CachePoolError)?;

        let _: () = conn
            .del(key)
            .await
            .map_err(CacheError::RedisExecutionFailed)?;

        Ok(())
    }
    pub async fn get_string(&self, key: &str) -> Result<Option<String>, CacheError> {
        match self.get(key).await {
            Ok(bytes) => {
                let s = String::from_utf8(bytes).map_err(CacheError::InvalidUtf8)?;
                Ok(Some(s))
            }
            Err(CacheError::CacheNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
