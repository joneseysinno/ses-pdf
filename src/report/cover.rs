use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoverPage {
    pub title: String,
    pub subtitle: Option<String>,
    /// Raw PNG bytes for a company logo displayed at the top of the cover.
    #[serde(skip)]
    pub logo_png: Option<Vec<u8>>,
    /// Ordered key-value pairs shown in the project info table.
    pub project_info: Vec<(String, String)>,
    /// Revision table rows: (revision, date, description, initials).
    pub revisions: Vec<(String, String, String, String)>,
}
