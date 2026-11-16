pub mod engine;
pub mod error;
pub use engine::CacheEngine;
pub use error::CacheError;
use serde::{Serialize, de::DeserializeOwned};

pub trait Cacheable: Serialize + DeserializeOwned {
    type Key: std::fmt::Display;
    fn namespace() -> &'static str;
    fn ttl() -> i64;
}
