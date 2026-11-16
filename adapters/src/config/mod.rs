pub mod env;
pub mod error;
use dotenvy::{dotenv, from_path};
pub use env::AppEnv;
pub use error::ConfigError;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct RawConfig {
    app_env: AppEnv,
    database_url: String,
    database_app_url: String,
    redis_app_url: String,
    jwt_secret: String,
}
#[derive(Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub database_url: String,
    pub database_migrator_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn new(path_str: &str) -> Result<Self, ConfigError> {
        let path = Path::new(path_str);
        if path.exists() {
            let _ = from_path(path);
        } else {
            let _ = dotenv();
        }
        let raw: RawConfig = envy::from_env()?;
        Ok(Self {
            app_env: raw.app_env,
            database_url: raw.database_app_url,
            database_migrator_url: raw.database_url,
            redis_url: raw.redis_app_url,
            jwt_secret: raw.jwt_secret,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_variables,
        unused_doc_comments,
        unused_mut,
        dead_code,
        unused_imports
    )]
    use super::*;

    #[test]
    fn test_valid_parsing() {
        assert_eq!(
            "development".parse::<AppEnv>().unwrap(),
            AppEnv::Development
        );
        println!("Testing for dev value");
        assert_eq!("dev".parse::<AppEnv>().unwrap(), AppEnv::Development);
        println!("Testing for staging value");
        assert_eq!("staging".parse::<AppEnv>().unwrap(), AppEnv::Staging);
        assert_eq!("stage".parse::<AppEnv>().unwrap(), AppEnv::Staging);
        assert_eq!("production".parse::<AppEnv>().unwrap(), AppEnv::Production);
        assert_eq!("prod".parse::<AppEnv>().unwrap(), AppEnv::Production);
    }

    #[test]
    fn test_case_and_whitespace_insensitivity() {
        assert_eq!("  DEV  ".parse::<AppEnv>().unwrap(), AppEnv::Development);
        assert_eq!("PROD".parse::<AppEnv>().unwrap(), AppEnv::Production);
        assert_eq!("StAgInG".parse::<AppEnv>().unwrap(), AppEnv::Staging);
    }

    #[test]
    fn test_invalid_env_fails() {
        let result = "invalid_env".parse::<AppEnv>();
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from() {
        assert_eq!(AppEnv::try_from("dev").unwrap(), AppEnv::Development);
        assert_eq!(
            AppEnv::try_from("prod".to_string()).unwrap(),
            AppEnv::Production
        );
    }

    #[test]
    fn test_helper_methods() {
        let env = AppEnv::Production;
        assert!(env.is_production());
        assert!(!env.is_development());
        assert!(!env.is_staging());
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#""DEVELOPMENT""#;
        let env: AppEnv = serde_json::from_str(json).unwrap();
        assert_eq!(env, AppEnv::Development);
    }
}
