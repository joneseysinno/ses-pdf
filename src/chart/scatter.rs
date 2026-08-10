use plotters::prelude::*;
use plotters_svg::SVGBackend;
use serde::{Deserialize, Serialize};

use super::color::{ChartColor, DEFAULT_PALETTE};
use super::ChartError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatterChart {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub series: Vec<ScatterSeries>,
    pub width: u32,
    pub height: u32,
    pub x_range: Option<(f64, f64)>,
    pub y_range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatterSeries {
    pub name: String,
    pub points: Vec<(f64, f64)>,
    pub color: Option<ChartColor>,
}

pub(crate) fn render_scatter(c: &ScatterChart) -> Result<String, ChartError> {
    let mut buf = String::new();
    {
        let root = SVGBackend::with_string(&mut buf, (c.width, c.height)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError(e.to_string()))?;

        let (x_min, x_max) = c.x_range.unwrap_or_else(|| auto_range_x(&c.series));
        let (y_min, y_max) = c.y_range.unwrap_or_else(|| auto_range_y(&c.series));

        let mut chart = ChartBuilder::on(&root)
            .caption(&c.title, ("sans-serif", 18u32).into_font())
            .margin(15u32)
            .x_label_area_size(45u32)
            .y_label_area_size(55u32)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| ChartError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&c.x_label)
            .y_desc(&c.y_label)
            .draw()
            .map_err(|e| ChartError(e.to_string()))?;

        for (i, series) in c.series.iter().enumerate() {
            let color: RGBColor = series
                .color
                .unwrap_or(DEFAULT_PALETTE[i % DEFAULT_PALETTE.len()])
                .into();
            let pts = series.points.clone();
            let name = series.name.clone();
            chart
                .draw_series(pts.iter().map(|&(x, y)| {
                    Circle::new((x, y), 4u32, color.filled())
                }))
                .map_err(|e| ChartError(e.to_string()))?
                .label(name)
                .legend(move |(x, y)| Circle::new((x + 7, y), 4u32, color.filled()));
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

fn auto_range_x(series: &[ScatterSeries]) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for s in series {
        for (x, _) in &s.points {
            if *x < min {
                min = *x;
            }
            if *x > max {
                max = *x;
            }
        }
    }
    if min >= max { (0.0, 1.0) } else { (min, max) }
}

fn auto_range_y(series: &[ScatterSeries]) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for s in series {
        for (_, y) in &s.points {
            if *y < min {
                min = *y;
            }
            if *y > max {
                max = *y;
            }
        }
    }
    if min >= max {
        (0.0, 1.0)
    } else {
        let pad = (max - min) * 0.05;
        (min - pad, max + pad)
    }
}
