use super::ContentError;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum ContentStatus {
    Draft,
    Publish,
    Archieve,
}

impl FromStr for ContentStatus {
    type Err = ContentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "draft" => Ok(Self::Draft),
            "publish" => Ok(Self::Publish),
            "archieve" => Ok(Self::Archieve),
            other => Err(ContentError::StatusInvalid(format!(
                "Invalid Status: '{other}'. Expected: draft, publish, archieve"
            ))),
        }
    }
}

impl TryFrom<String> for ContentStatus {
    type Error = ContentError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<&str> for ContentStatus {
    type Error = ContentError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
