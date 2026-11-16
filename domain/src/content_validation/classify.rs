use super::ContentError;
use super::Frontmatter;
use crate::content_validation::frontmatter::RawFrontmatter;
use yaml_serde;

pub struct Content {
    pub metadata: Frontmatter,
    pub body: String,
}

pub enum ClassifyResult {
    Invalid { error: String },
    Valid { content: Content },
    Skipped { reason: String },
    Incomplete { missing: String },
}

pub fn classify(entry: &str) -> Result<ClassifyResult, ContentError> {
    if entry.is_empty() {
        return Ok(ClassifyResult::Skipped {
            reason: "Empty File".to_string(),
        });
    }
    let parts: Vec<&str> = entry.splitn(3, "---").collect();
    if parts.len() == 3 {
        let body = parts[3].trim().to_string();
        let raw_trim = parts[1].trim();
        let raw_metadata: RawFrontmatter =
            yaml_serde::from_str(raw_trim).map_err(ContentError::FrontMatterInvalid)?;
        let metadata = Frontmatter::validation(raw_metadata);
        match metadata {
            Ok(m) => {
                let content = Content { metadata: m, body };
                Ok(ClassifyResult::Valid { content })
            }
            Err(e) => Ok(ClassifyResult::Incomplete {
                missing: e.to_string(),
            }),
        }
    } else {
        Ok(ClassifyResult::Invalid {
            error: "No defining separator for metadata and content".to_string(),
        })
    }
}
