mod calc_block;
mod cover;
mod equation;
pub(crate) mod escape;
mod figure;
mod heading;
mod page;
mod table;

use crate::element::ReportElement;
use crate::report::Report;
use crate::world::VirtualFs;
use crate::PdfError;

pub(crate) fn generate_source(report: &Report) -> Result<(String, VirtualFs), PdfError> {
    let mut out = String::with_capacity(32_768);
    let mut vfs = VirtualFs::new();
    let mut chart_idx: usize = 0;
    let mut figure_idx: usize = 0;

    // Page setup
    out.push_str(&page::generate_page_setup(&report.metadata));

    // Cover page
    if let Some(cov) = &report.cover {
        let has_logo = cov.logo_png.is_some();
        if let Some(logo_bytes) = &cov.logo_png {
            vfs.insert("logo.png", logo_bytes.clone());
        }
        out.push_str(&cover::generate_cover(cov, has_logo));
    }

    // Sections
    for section in &report.sections {
        out.push_str(&heading::generate_heading(&section.title, section.level));
        for element in &section.elements {
            push_element(&mut out, &mut vfs, element, &mut chart_idx, &mut figure_idx)?;
        }
    }

    Ok((out, vfs))
}

fn push_element(
    out: &mut String,
    vfs: &mut VirtualFs,
    element: &ReportElement,
    chart_idx: &mut usize,
    figure_idx: &mut usize,
) -> Result<(), PdfError> {
    match element {
        ReportElement::Heading(h) => {
            out.push_str(&heading::generate_heading(&h.text, h.level));
        }
        ReportElement::Paragraph(p) => {
            out.push_str(&format!("\n{}\n", escape::escape_text(p)));
        }
        ReportElement::Table(t) => {
            out.push_str(&table::generate_table(t));
        }
        ReportElement::Equation(eq) => {
            out.push_str(&equation::generate_equation(eq));
        }
        ReportElement::CalcBlock(cb) => {
            out.push_str(&calc_block::generate_calc_block(cb));
        }
        ReportElement::Chart { chart, caption } => {
            let name = format!("chart-{chart_idx}.svg");
            *chart_idx = chart_idx
                .checked_add(1)
                .ok_or(PdfError::Builder("too many charts"))?;
            let svg = crate::chart::render_chart_to_svg(chart)
                .map_err(|e| PdfError::Chart(e.0))?;
            vfs.insert(&name, svg.into_bytes());
            out.push_str(&figure::generate_svg_figure(&name, caption));
        }
        ReportElement::Figure(fig) => {
            let name = format!("figure-{figure_idx}.{}", fig.format.file_extension());
            *figure_idx = figure_idx
                .checked_add(1)
                .ok_or(PdfError::Builder("too many figures"))?;
            vfs.insert(&name, fig.data.clone());
            out.push_str(&figure::generate_static_figure(&name, fig));
        }
        ReportElement::PageBreak => {
            out.push_str("\n#pagebreak()\n");
        }
        ReportElement::HorizontalRule => {
            out.push_str("\n#line(length: 100%)\n");
        }
    }
    Ok(())
}
