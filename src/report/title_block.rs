use serde::{Deserialize, Serialize};

/// Drawing-sheet title block shown in the page footer chrome.
///
/// When attached to a [`Report`](super::Report), page setup switches from
/// calculation-report headers/footers to drawing-sheet margins, border, and
/// this title block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleBlock {
    /// Sheet title, e.g. `"Foundation / Framing Plan"`.
    pub sheet_title: String,
    /// Sheet number, e.g. `"S-101"`.
    pub sheet_number: String,
    /// Drawing scale, e.g. `"1/4\" = 1'-0\""`.
    pub scale: String,
    pub drawn_by: String,
    pub checked_by: Option<String>,
    /// Revision rows: `(revision, date, description)`.
    pub revisions: Vec<(String, String, String)>,
}
