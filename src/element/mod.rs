mod calc_block;
mod equation;
mod figure;
mod heading;
mod table;

pub use calc_block::{CalcBlock, CalcBlockBuilder, CalcStatus};
pub use equation::Equation;
pub use figure::{Figure, ImageFormat};
pub use heading::{Heading, HeadingLevel};
pub use table::{CellAlign, CellValue, ColumnSpec, ColumnWidth, Table, TableBuilder};

use serde::{Deserialize, Serialize};

/// All content items that can appear within a [`super::report::Section`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportElement {
    Heading(Heading),
    Paragraph(String),
    Table(Table),
    Equation(Equation),
    CalcBlock(CalcBlock),
    Chart {
        chart: crate::chart::Chart,
        caption: String,
    },
    Figure(Figure),
    PageBreak,
    HorizontalRule,
}

impl From<Heading> for ReportElement {
    fn from(h: Heading) -> Self {
        Self::Heading(h)
    }
}

impl From<Table> for ReportElement {
    fn from(t: Table) -> Self {
        Self::Table(t)
    }
}

impl From<Equation> for ReportElement {
    fn from(e: Equation) -> Self {
        Self::Equation(e)
    }
}

impl From<CalcBlock> for ReportElement {
    fn from(c: CalcBlock) -> Self {
        Self::CalcBlock(c)
    }
}

impl From<Figure> for ReportElement {
    fn from(f: Figure) -> Self {
        Self::Figure(f)
    }
}
