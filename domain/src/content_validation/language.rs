use super::ContentError;
use language_tags::LanguageTag;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ContentLanguage(LanguageTag);

impl ContentLanguage {
    pub fn parse(s: &str) -> Result<Self, ContentError> {
        let tag = LanguageTag::parse(s)?;
        Ok(Self(tag))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
