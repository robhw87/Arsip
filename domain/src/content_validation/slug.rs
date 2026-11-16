use super::ContentError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Slug(String);

impl Slug {
    pub fn parse(raw: String) -> Result<Self, ContentError> {
        let trimmed = raw.trim().to_lowercase();
        if trimmed.is_empty() {
            Err(ContentError::SlugEmptyInput)
        } else {
            let sluggify = trimmed.replace(' ', "-");
            if !sluggify.chars().all(|c| c.is_alphanumeric() || c == '-') {
                Err(ContentError::SlugFormat(raw))
            } else {
                Ok(Self(sluggify))
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ContentError;
    use super::Slug;
    #[test]
    fn slug_std_test() {
        assert_eq!(
            Slug::parse("hello world".to_string()).unwrap().as_str(),
            "hello-world"
        );
    }

    #[test]
    fn slug_whitespace_test() {
        assert_eq!(
            Slug::parse("  hello world  ".to_string()).unwrap().as_str(),
            "hello-world"
        );
    }
    #[test]
    fn slug_empty_input_is_rejected() {
        assert!(matches!(
            Slug::parse("".to_string()),
            Err(ContentError::SlugEmptyInput)
        ));
    }

    #[test]
    fn slug_whitespace_only_input_is_rejected() {
        assert!(matches!(
            Slug::parse("   ".to_string()),
            Err(ContentError::SlugEmptyInput)
        ));
    }
}
