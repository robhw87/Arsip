pub mod input;
pub mod output;
pub mod process;

pub struct SiteIndex {
    pub entries: Vec<Entry>,
    pub by_slug: BTreeMap<Slug, EntryId>,
    pub tags: BTreeMap<Tag, Vec<EntryId>>,
}

pub enum Entry {
    Post(Post),
    Project(Project),
    Page(Page),
}

pub struct Post {
    pub id: EntryId,
    pub meta: PostMeta,
    pub body: RenderedBody,
}

pub struct PostMeta {
    pub title: String,
    pub slug: Slug,
    pub date: chrono::NaiveDate,
    pub status: Status,
    pub tags: Vec<Tag>,
    pub css_class: Option<CssClass>,
}

pub enum Status {
    Draft,
    Published,
}

pub struct RenderedBody {
    pub html: String, // from the Markdown parser
    pub toc: Vec<TocItem>,
    pub reading_time_min: u32,
    pub excerpt: String,
}

pub struct SiteConfig {
    pub title: String,
    pub base_url: Url,
    pub author: String,
    pub nav: Vec<NavLink>,
    pub tokens: DesignTokens,
    pub paths: ContentPaths,
}

pub struct DesignTokens {
    pub accent: HexColor,
    pub content_width: CssLength,
    pub font_body: FontChoice, // enum from an allowlist
}

fn build_index(entries: Vec<Entry>) -> Result<SiteIndex, CmsError>; // catches duplicate slugs
fn validate(doc: ParsedDocument) -> Result<Entry, CmsError>;
fn render_body(markdown: &str) -> RenderedBody; // Markdown to HTML

pub struct ParsedDocument {
    pub source: SourcePath,
    pub kind: ContentKind,
    pub frontmatter: RawFrontmatter, // untyped keys and values from serde
    pub markdown: String,
}

pub enum ContentKind {
    Post,
    Project,
    Page,
}
