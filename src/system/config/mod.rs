pub mod env;
pub mod error;
use dotenvy::{dotenv, from_path};
use env::AppEnv;
use error::ConfigError;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct RawConfig {
    app_env: AppEnv,
    server_host: String,
    server_port: u32,
    db_dbs: String,
    db_host: String,
    db_port: u32,
    db_migrator: String,
    db_migrator_pass: String,
    db_app_user: String,
    db_app_user_pass: String,
    db_max_conn: Option<u32>,
    db_min_conn: Option<u32>,
    db_conn_timeout: Option<u64>,
    redis_host: String,
    redis_port: u32,
    redis_app_pass: String,
    jwt_secret: String,
    jwt_exp_mins: Option<u32>,
}
#[derive(Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub database_url: String,
    pub database_migrator_url: String,
    pub redis_url: String,
    pub db_max_conn: u32,
    pub db_min_conn: u32,
    pub db_conn_timeout: u64,
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
        let (default_max, default_min, default_timeout) = match raw.app_env {
            AppEnv::Development => (5, 0, 5),
            AppEnv::Staging => (10, 2, 5),
            AppEnv::Production => (20, 5, 15),
        };
        let database_migrator_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            raw.db_migrator, raw.db_migrator_pass, raw.db_host, raw.db_port, raw.db_dbs
        );
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            raw.db_app_user, raw.db_app_user_pass, raw.db_host, raw.db_port, raw.db_dbs
        );
        let redis_url = format!(
            "redis://app:{}@{}:{}/0",
            raw.redis_app_pass, raw.redis_host, raw.redis_port
        );
        Ok(Self {
            app_env: raw.app_env,
            database_url,
            database_migrator_url,
            redis_url,
            db_max_conn: raw.db_max_conn.unwrap_or(default_max),
            db_min_conn: raw.db_min_conn.unwrap_or(default_min),
            db_conn_timeout: raw.db_conn_timeout.unwrap_or(default_timeout),
            jwt_secret: raw.jwt_secret,
        })
    }
}

#[cfg(test)]
#[derive(Deserialize, Debug)]
pub struct TestHelper {
    app_env: AppEnv,
    database_url: String,
    database_migrator_url: String,
    database_max_connections: Option<u32>,
    database_min_connections: Option<u32>,
    jwt_secret: String,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_read() -> Result<(), ConfigError> {
        let path = Path::new(".env");
        if path.exists() {
            let _ = from_path(path);
        } else {
            let _ = dotenv();
        }
        let raw: TestHelper = envy::from_env()?;
        println!("raw = {:?}", raw);
        Ok(())
    }
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
