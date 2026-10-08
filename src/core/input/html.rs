pub enum SaveOutcome {
    Saved { path: SourcePath, new_hash: ContentHash },
    Conflict { current_hash: ContentHash },       // file changed since it was opened
    Invalid(Vec<FieldError>),                     // shown inline in the form
}

pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}
