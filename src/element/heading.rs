use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub text: String,
    pub level: HeadingLevel,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
}

impl HeadingLevel {
    pub(crate) fn typst_prefix(self) -> &'static str {
        match self {
            Self::H1 => "=",
            Self::H2 => "==",
            Self::H3 => "===",
        }
    }
}
