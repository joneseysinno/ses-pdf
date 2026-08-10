use thiserror::Error;

/// All errors that can occur during report compilation.
#[derive(Debug, Error)]
pub enum PdfError {
    /// The Typst source generated from the document model contained a syntax or semantic error.
    #[error("typst compilation error: {0}")]
    Compile(String),

    /// The Typst PDF export step failed.
    #[error("typst PDF export error: {0}")]
    Export(String),

    /// A chart could not be rendered to SVG.
    #[error("chart render error: {0}")]
    Chart(String),

    /// Font loading from the bundled asset store failed.
    #[error("font loading error: {0}")]
    FontLoad(String),

    /// A required field was not set on a builder.
    #[error("incomplete builder: {0}")]
    Builder(&'static str),
}
