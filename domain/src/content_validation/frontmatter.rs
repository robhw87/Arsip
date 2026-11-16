use super::{ContentError, ContentLanguage, ContentStatus, Slug};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Frontmatter {
    pub slug: Slug,
    pub title: String,
    pub language: ContentLanguage,
    pub status: ContentStatus,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub aliases: Vec<Slug>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RawFrontmatter {
    pub title: String,
    pub slug: Option<String>,
    pub language: Option<String>,
    pub status: ContentStatus,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
}
impl Frontmatter {
    pub fn validation(raw: RawFrontmatter) -> Result<Self, ContentError> {
        let title = raw.title.trim().to_string();
        let slug = if let Some(sl) = raw.slug {
            Slug::parse(sl)
        } else {
            Slug::parse(title.clone())
        }?;
        let aliases = if let Some(als) = raw.aliases {
            let mut vc = Vec::new();
            for al in als {
                let rslt = Slug::parse(al)?;
                vc.push(rslt);
            }
            vc
        } else {
            Vec::new()
        };
        let language = match raw.language {
            Some(s) => ContentLanguage::parse(&s),
            None => ContentLanguage::parse("en-ID"),
        }?;
        Ok(Self {
            title,
            slug,
            language,
            status: raw.status,
            category: raw.category,
            tags: raw.tags,
            aliases,
        })
    }
}
