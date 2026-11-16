pub mod cms;
pub mod error;
pub mod init;
use crate::cache::Cacheable;
pub use error::DbError;
pub use init::AppDbPool;

pub trait DatabaseCacheable: Cacheable + Sized {
    fn fetch_db(
        db_pool: AppDbPool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, DbError>> + Send;
}
