pub mod error;
pub mod redis;
pub use redis::CacheEngine;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait Cacheable: Serialize + DeserializeOwned {
    type Key: std::fmt::Display;
    fn namespace() -> &'static str;
    fn ttl() -> i64;
}
