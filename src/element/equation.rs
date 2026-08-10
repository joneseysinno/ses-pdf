use serde::{Deserialize, Serialize};

/// A mathematical equation using Typst math notation (not LaTeX).
///
/// Typst math differs from LaTeX: `phi` not `\phi`, `a/b` not `\frac{a}{b}`,
/// display math uses `$ expr $` (spaces inside the `$`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    /// The Typst math expression (no surrounding `$`).
    pub math: String,
    /// `true` → block/display equation; `false` → inline.
    pub display: bool,
    /// Optional label for cross-referencing, e.g. `"eq:moment-capacity"`.
    pub label: Option<String>,
}

impl Equation {
    pub fn display(math: impl Into<String>) -> Self {
        Self {
            math: math.into(),
            display: true,
            label: None,
        }
    }

    pub fn inline(math: impl Into<String>) -> Self {
        Self {
            math: math.into(),
            display: false,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}
