use crate::system::AppState;
use crate::system::cache::Cacheable;
use crate::system::db::{DatabaseCacheable, DbError};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;
use std::time::Duration;
use time::{Date, OffsetDateTime};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DbMetadata {
    pub filename: String,
    pub slug: String,
    pub file_path: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub published_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub summary: String,
}
impl DbMetadata {
    pub async fn get_post(state: &AppState, slug: &str) -> Result<Option<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as!(
            Self,
            r#"
            SELECT 
               filename, 
               slug,
               file_path,
               title,
               category,
               status,
               published_at,
               updated_at,
               summary
            FROM cms.posts
            WHERE status = 'publish' AND 
            slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;
        Ok(datas)
    }
    pub async fn get_all(state: &AppState) -> Result<Vec<Self>, DbError> {
        let pool = state.pool();
        let datas = sqlx::query_as!(
            Self,
            r#"
            SELECT 
               filename, 
               slug,
               file_path,
               title,
               category,
               status,
               published_at,
               updated_at,
               summary
            FROM cms.posts
            WHERE status = 'publish'
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(datas)
    }
}

impl Cacheable for DbMetadata {
    type Key = String;

    fn namespace() -> &'static str {
        "metadata" // Saved to ./cache_dir/scans/<key>.json
    }

    fn ttl() -> i64 {
        60 * 60 * 12 // Fresh for 1 hour
    }
}
