//! Professional PDF report engine for structural engineering calculations.
//!
//! `ses-pdf` compiles a structured document model into a publication-quality PDF
//! using the [Typst](https://typst.app) typesetting engine under the hood.
//! Reports can contain:
//!
//! - Engineering equations with Typst math notation
//! - Calculation blocks (formula → substitution → numeric result)
//! - Data tables with shaded headers
//! - Line, bar, scatter, and P-M interaction charts (vector SVG)
//! - User-supplied images and SVG figures
//! - Cover page, section numbering, headers, footers, and page numbers
//!
//! # Quick start
//!
//! ```rust,no_run
//! use ses_pdf::*;
//!
//! let pdf_bytes = Report::new(ReportMetadata {
//!     project_name:   "Grid B5 Beam".into(),
//!     project_number: "2025-001".into(),
//!     prepared_by:    "SES Engineering".into(),
//!     checked_by:     None,
//!     date:           "March 14, 2025".into(),
//!     paper:          PaperSize::Letter,
//! })
//! .section(
//!     Section::new("Moment Capacity")
//!         .text("Nominal moment capacity per ACI 318-19 Section 22.2.")
//!         .equation(Equation::display("phi M_n = phi dot A_s dot f_y dot (d - a/2)"))
//!         .calc(
//!             CalcBlock::new("Nominal Moment Capacity")
//!                 .formula("M_n = A_s f_y (d - a/2)")
//!                 .substitution("M_n = (0.60)(60)(14.5 - 1.25)")
//!                 .result("M_n = 477 \"kip-in\"")
//!                 .build()
//!                 .unwrap(),
//!         ),
//! )
//! .render()
//! .unwrap();
//!
//! std::fs::write("report.pdf", pdf_bytes).unwrap();
//! ```
//!
//! # Math notation
//!
//! Equations use **Typst math syntax**, not LaTeX:
//!
//! | LaTeX | Typst |
//! |-------|-------|
//! | `\phi` | `phi` |
//! | `\frac{a}{b}` | `a/b` |
//! | `\sqrt{x}` | `sqrt(x)` |
//! | `\times` | `times` |
//! | Display math `\[ ... \]` | `$ ... $` (spaces inside) |
//! | `A_{cv}` (multi-letter sub) | `A_"cv"` |
//!
//! See the [Typst math documentation](https://typst.app/docs/reference/math/) for the full reference.

#![forbid(unsafe_code)]

pub mod chart;
pub mod element;

mod error;
mod markup;
mod report;
mod world;

pub use chart::{
    BarChart, BarSeries, Chart, ChartColor, DemandPoint, LineChart, LineSeries,
    PmInteractionDiagram, ScatterChart, ScatterSeries,
};
pub use element::{
    CalcBlock, CalcBlockBuilder, CalcStatus, CellAlign, CellValue, ColumnSpec, ColumnWidth,
    Equation, Figure, HeadingLevel, ImageFormat, Table, TableBuilder,
};
pub use error::PdfError;
pub use report::{CoverPage, PaperSize, Report, ReportMetadata, Section};
