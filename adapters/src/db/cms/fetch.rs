use crate::cache::Cacheable;
use crate::db::AppDbPool;
use crate::db::DbError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

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
    pub async fn get_post(db_pool: AppDbPool, slug: &str) -> Result<Option<Self>, DbError> {
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
        .fetch_optional(db_pool)
        .await?;
        Ok(datas)
    }
    pub async fn get_all(db_pool: AppDbPool) -> Result<Vec<Self>, DbError> {
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
        .fetch_all(db_pool)
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
