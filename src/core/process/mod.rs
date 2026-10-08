pub mod slug;
pub use slug::Slug;
fn render_page(entry: &Entry, index: &SiteIndex, cfg: &SiteConfig) -> OutputFile {
    todo!()
}
fn build_feed(index: &SiteIndex, cfg: &SiteConfig) -> OutputFile;
fn build_sitemap(index: &SiteIndex, cfg: &SiteConfig) -> OutputFile;
fn build_search_index(index: &SiteIndex) -> OutputFile;
fn bundle_styles(styles: &StyleSources, cfg: &SiteConfig) -> Result<OutputFile, CmsError>;
fn process_images(images: &[RawImage]) -> Vec<OutputFile>;
