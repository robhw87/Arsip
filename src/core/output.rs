pub struct BuiltSite {
    pub files: Vec<OutputFile>,
    pub report: BuildReport,
}

pub struct OutputFile {
    pub path: OutputPath,      // newtype: relative, no "..", validated
    pub kind: OutputKind,
    pub bytes: Vec<u8>,
}

pub enum OutputKind {
    Html,
    Feed,          // feed.xml
    Sitemap,       // sitemap.xml
    SearchIndex,   // search-index.json
    Stylesheet,    // bundled, hashed CSS
    Image,         // optimized WebP variants
    Static,        // copied as-is
}

pub struct BuildReport {
    pub pages_built: usize,
    pub warnings: Vec<Warning>,
    pub duration: std::time::Duration,
}



