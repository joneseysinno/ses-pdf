use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub caption: String,
    /// Width as a percentage of the text column (1–100).
    pub width_pct: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Svg,
}

impl ImageFormat {
    pub(crate) fn file_extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Svg => "svg",
        }
    }
}
