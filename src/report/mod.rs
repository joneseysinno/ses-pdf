mod cover;
mod metadata;
mod section;

pub use cover::CoverPage;
pub use metadata::{PaperSize, ReportMetadata};
pub use section::Section;

use crate::PdfError;

pub struct Report {
    pub(crate) metadata: ReportMetadata,
    pub(crate) cover: Option<CoverPage>,
    pub(crate) sections: Vec<Section>,
}

impl Report {
    pub fn new(metadata: ReportMetadata) -> Self {
        Self {
            metadata,
            cover: None,
            sections: Vec::new(),
        }
    }

    pub fn cover(mut self, cover: CoverPage) -> Self {
        self.cover = Some(cover);
        self
    }

    pub fn section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    /// Compile the report to PDF bytes.
    pub fn render(self) -> Result<Vec<u8>, PdfError> {
        let (source, vfs) = crate::markup::generate_source(&self)?;
        let world = crate::world::SesWorld::new(source, vfs)?;

        let warned = typst::compile::<typst_layout::PagedDocument>(&world);
        let document = warned
            .output
            .map_err(|diags| PdfError::Compile(format_diagnostics(&diags)))?;

        typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default())
            .map_err(|diags| PdfError::Export(format_diagnostics(&diags)))
    }
}

fn format_diagnostics(
    diags: &typst::ecow::EcoVec<typst::diag::SourceDiagnostic>,
) -> String {
    diags
        .iter()
        .map(|d| d.message.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}
