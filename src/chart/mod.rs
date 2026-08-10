mod bar;
mod color;
mod line;
mod pm_interaction;
mod scatter;

pub use bar::{BarChart, BarSeries};
pub use color::ChartColor;
pub use line::{LineChart, LineSeries};
pub use pm_interaction::{DemandPoint, PmInteractionDiagram};
pub use scatter::{ScatterChart, ScatterSeries};

use serde::{Deserialize, Serialize};

/// A chart that can be embedded in a report section.
///
/// Charts are rendered to SVG by plotters and embedded as vector graphics in the PDF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chart {
    Line(LineChart),
    Bar(BarChart),
    Scatter(ScatterChart),
    PmInteraction(PmInteractionDiagram),
}

#[derive(Debug)]
pub(crate) struct ChartError(pub String);

pub(crate) fn render_chart_to_svg(chart: &Chart) -> Result<String, ChartError> {
    match chart {
        Chart::Line(c) => line::render_line(c),
        Chart::Bar(c) => bar::render_bar(c),
        Chart::Scatter(c) => scatter::render_scatter(c),
        Chart::PmInteraction(c) => pm_interaction::render_pm(c),
    }
}
