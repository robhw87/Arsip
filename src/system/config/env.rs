use crate::system::config::error::ConfigError;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum AppEnv {
    Development,
    Staging,
    Production,
}

impl AppEnv {
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    pub fn is_staging(&self) -> bool {
        matches!(self, Self::Staging)
    }

    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }
}

impl FromStr for AppEnv {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "dev" | "development" => Ok(Self::Development),
            "stage" | "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Production),
            other => Err(ConfigError::Config(format!(
                "Invalid APP_ENV: '{other}'. Expected: dev, staging, prod"
            ))),
        }
    }
}

// 2. TryFrom implementations delegate directly to FromStr
impl TryFrom<String> for AppEnv {
    type Error = ConfigError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<&str> for AppEnv {
    type Error = ConfigError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
