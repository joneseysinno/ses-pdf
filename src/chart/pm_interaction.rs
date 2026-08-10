use plotters::prelude::*;
use plotters_svg::SVGBackend;
use serde::{Deserialize, Serialize};

use super::ChartError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmInteractionDiagram {
    pub title: String,
    /// Interaction curve: (moment M, axial force P) pairs tracing the envelope.
    pub envelope: Vec<(f64, f64)>,
    /// Demand points that must fall inside the envelope.
    pub demand_points: Vec<DemandPoint>,
    pub m_label: String,
    pub p_label: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPoint {
    pub m: f64,
    pub p: f64,
    pub label: String,
}

pub(crate) fn render_pm(c: &PmInteractionDiagram) -> Result<String, ChartError> {
    let mut buf = String::new();
    {
        let root = SVGBackend::with_string(&mut buf, (c.width, c.height)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError(e.to_string()))?;

        let (m_min, m_max, p_min, p_max) = bounds(&c.envelope, &c.demand_points);

        let mut chart = ChartBuilder::on(&root)
            .caption(&c.title, ("sans-serif", 16u32).into_font())
            .margin(20u32)
            .x_label_area_size(50u32)
            .y_label_area_size(60u32)
            .build_cartesian_2d(m_min..m_max, p_min..p_max)
            .map_err(|e| ChartError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&c.m_label)
            .y_desc(&c.p_label)
            .draw()
            .map_err(|e| ChartError(e.to_string()))?;

        // Draw filled envelope polygon.
        let envelope_color = RGBColor(63, 127, 191);
        let fill_color = RGBAColor(63, 127, 191, 0.15);

        if !c.envelope.is_empty() {
            let mut closed = c.envelope.clone();
            if let Some(first) = c.envelope.first() {
                closed.push(*first);
            }
            chart
                .draw_series(std::iter::once(Polygon::new(
                    c.envelope.clone(),
                    fill_color,
                )))
                .map_err(|e| ChartError(e.to_string()))?;

            chart
                .draw_series(plotters::series::LineSeries::new(
                    closed,
                    envelope_color.stroke_width(2),
                ))
                .map_err(|e| ChartError(e.to_string()))?
                .label("Capacity Envelope")
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], envelope_color)
                });
        }

        // Draw demand points.
        for dp in &c.demand_points {
            let inside = point_inside_polygon(dp.m, dp.p, &c.envelope);
            let color = if inside {
                RGBColor(51, 153, 51)
            } else {
                RGBColor(204, 51, 51)
            };
            chart
                .draw_series(std::iter::once(Circle::new(
                    (dp.m, dp.p),
                    6u32,
                    color.filled(),
                )))
                .map_err(|e| ChartError(e.to_string()))?;
        }

        chart
            .configure_series_labels()
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()
            .map_err(|e| ChartError(e.to_string()))?;

        root.present().map_err(|e| ChartError(e.to_string()))?;
    }
    Ok(buf)
}

fn bounds(
    envelope: &[(f64, f64)],
    demands: &[DemandPoint],
) -> (f64, f64, f64, f64) {
    let mut m_min = f64::MAX;
    let mut m_max = f64::MIN;
    let mut p_min = f64::MAX;
    let mut p_max = f64::MIN;

    for (m, p) in envelope {
        if *m < m_min { m_min = *m; }
        if *m > m_max { m_max = *m; }
        if *p < p_min { p_min = *p; }
        if *p > p_max { p_max = *p; }
    }
    for dp in demands {
        if dp.m < m_min { m_min = dp.m; }
        if dp.m > m_max { m_max = dp.m; }
        if dp.p < p_min { p_min = dp.p; }
        if dp.p > p_max { p_max = dp.p; }
    }

    if m_min >= m_max { m_min = 0.0; m_max = 1.0; }
    if p_min >= p_max { p_min = 0.0; p_max = 1.0; }

    let mx_pad = (m_max - m_min) * 0.1;
    let py_pad = (p_max - p_min) * 0.1;
    (m_min - mx_pad, m_max + mx_pad, p_min - py_pad, p_max + py_pad)
}

/// Ray-casting algorithm to test if (px, py) is inside a polygon.
fn point_inside_polygon(px: f64, py: f64, polygon: &[(f64, f64)]) -> bool {
    let n = polygon.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        let intersect = ((yi > py) != (yj > py))
            && (px < (xj - xi) * (py - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}
