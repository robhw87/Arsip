use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Error : {0}")]
    GeneralError(String),
    #[error("Error : {0}")]
    FrontMatterInvalid(#[from] yaml_serde::Error),
    #[error(
        "Invalid slug format: '{0}'. Slugs must contain only lowercase alphanumeric characters and hyphens."
    )]
    SlugFormat(String),
    #[error("Empty Input For Slug")]
    SlugEmptyInput,
    #[error("Content Status Invalid")]
    StatusInvalid(String),
    #[error("Language Invalid: {0}")]
    LanguageInvalid(#[from] language_tags::ParseError),
}
