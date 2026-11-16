use crate::content_validation::classify::ClassifyResult;
pub struct ScanReport(Vec<ClassifyResult>);

impl ScanReport {
    pub fn new(input: Vec<ClassifyResult>) -> Self {
        let result = input?;
        Ok(ScanReport(result))
    }
    pub fn summary(&self) -> ScanSummary {  }
    pub fn exit_code(&self) -> ScanExitCode { ... }

    /// All categories referenced across valid + incomplete files.
    pub fn categories_found(&self) -> BTreeSet<CategoryName> { ... }

    /// All tags referenced across valid + incomplete files.
    pub fn tags_found(&self) -> BTreeSet<TagName> { ... }

    /// All aliases referenced across valid + incomplete files.
    pub fn aliases_found(&self) -> BTreeSet<Slug> { ... }

    /// Valid files only — what `commit` would consider ingesting.
    pub fn valid_files(&self) -> impl Iterator<Item = &ScannedFile> { ... }
}
