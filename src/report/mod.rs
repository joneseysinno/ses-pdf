mod cover;
mod metadata;
mod section;
mod title_block;

pub use cover::CoverPage;
pub use metadata::{Orientation, PaperSize, ReportMetadata};
pub use section::Section;
pub use title_block::TitleBlock;

use crate::PdfError;

pub struct Report {
    pub(crate) metadata: ReportMetadata,
    pub(crate) cover: Option<CoverPage>,
    pub(crate) title_block: Option<TitleBlock>,
    pub(crate) sections: Vec<Section>,
}

impl Report {
    pub fn new(metadata: ReportMetadata) -> Self {
        Self {
            metadata,
            cover: None,
            title_block: None,
            sections: Vec::new(),
        }
    }

    pub fn cover(mut self, cover: CoverPage) -> Self {
        self.cover = Some(cover);
        self
    }

    /// Attach a drawing-sheet title block. Switches page chrome to drawing mode.
    pub fn title_block(mut self, title_block: TitleBlock) -> Self {
        self.title_block = Some(title_block);
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
