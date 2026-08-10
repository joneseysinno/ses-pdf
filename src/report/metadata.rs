use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub project_name: String,
    pub project_number: String,
    pub prepared_by: String,
    pub checked_by: Option<String>,
    /// Date string displayed on the report, e.g. `"August 10, 2026"`.
    pub date: String,
    pub paper: PaperSize,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum PaperSize {
    #[default]
    Letter,
    Tabloid,
    A4,
}

impl PaperSize {
    pub(crate) fn typst_name(self) -> &'static str {
        match self {
            Self::Letter => "us-letter",
            Self::Tabloid => "us-tabloid",
            Self::A4 => "a4",
        }
    }
}
