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
    /// Page orientation. Defaults to portrait for calculation reports.
    #[serde(default)]
    pub orientation: Orientation,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum PaperSize {
    #[default]
    Letter,
    Tabloid,
    A4,
    /// 9 × 12 in
    ArchA,
    /// 12 × 18 in
    ArchB,
    /// 18 × 24 in
    ArchC,
    /// 24 × 36 in
    ArchD,
    /// 36 × 48 in
    ArchE,
    /// 30 × 42 in
    ArchE1,
}

impl PaperSize {
    pub(crate) fn typst_name(self) -> &'static str {
        match self {
            Self::Letter => "us-letter",
            Self::Tabloid => "us-tabloid",
            Self::A4 => "a4",
            Self::ArchA => "arch-a",
            Self::ArchB => "arch-b",
            Self::ArchC => "arch-c",
            Self::ArchD => "arch-d",
            Self::ArchE => "arch-e",
            Self::ArchE1 => "arch-e1",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
}

impl Orientation {
    pub(crate) fn flipped(self) -> bool {
        matches!(self, Self::Landscape)
    }
}
