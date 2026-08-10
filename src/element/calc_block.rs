use serde::{Deserialize, Serialize};

/// Engineering calculation block showing formula → substitution → result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcBlock {
    pub label: String,
    /// Typst math for the formula, e.g. `"M_n = A_s f_y (d - a/2)"`.
    pub formula: String,
    /// Typst math with values substituted, e.g. `"M_n = (0.60)(60)(14.5 - 1.25)"`.
    pub substitution: String,
    /// Typst math result, e.g. `"M_n = 477 \"kip-in\""`.
    pub result: String,
    /// Optional narrative placed below the result.
    pub note: Option<String>,
    pub status: CalcStatus,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CalcStatus {
    #[default]
    Ok,
    Governs,
    DoesNotGovern,
}

impl CalcStatus {
    pub(crate) fn stroke_color(self) -> &'static str {
        match self {
            Self::Ok => "2c4c7c",
            Self::Governs => "28a745",
            Self::DoesNotGovern => "aaaaaa",
        }
    }
}

pub struct CalcBlockBuilder {
    label: String,
    formula: Option<String>,
    substitution: Option<String>,
    result: Option<String>,
    note: Option<String>,
    status: CalcStatus,
}

impl CalcBlock {
    pub fn new(label: impl Into<String>) -> CalcBlockBuilder {
        CalcBlockBuilder {
            label: label.into(),
            formula: None,
            substitution: None,
            result: None,
            note: None,
            status: CalcStatus::default(),
        }
    }
}

impl CalcBlockBuilder {
    pub fn formula(mut self, f: impl Into<String>) -> Self {
        self.formula = Some(f.into());
        self
    }

    pub fn substitution(mut self, s: impl Into<String>) -> Self {
        self.substitution = Some(s.into());
        self
    }

    pub fn result(mut self, r: impl Into<String>) -> Self {
        self.result = Some(r.into());
        self
    }

    pub fn note(mut self, n: impl Into<String>) -> Self {
        self.note = Some(n.into());
        self
    }

    pub fn status(mut self, s: CalcStatus) -> Self {
        self.status = s;
        self
    }

    pub fn build(self) -> Result<CalcBlock, crate::PdfError> {
        Ok(CalcBlock {
            label: self.label,
            formula: self
                .formula
                .ok_or(crate::PdfError::Builder("CalcBlock missing formula"))?,
            substitution: self
                .substitution
                .ok_or(crate::PdfError::Builder("CalcBlock missing substitution"))?,
            result: self
                .result
                .ok_or(crate::PdfError::Builder("CalcBlock missing result"))?,
            note: self.note,
            status: self.status,
        })
    }
}
