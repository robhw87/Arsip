pub mod error;
pub mod metadata;
use std::process::Output;

use crate::system::cache::Cacheable;
use crate::system::state::AppState;
pub use error::DbError;

pub trait DatabaseCacheable: Cacheable + Sized {
    fn fetch_db(
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, DbError>> + Send;
}
