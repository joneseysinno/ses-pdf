use plotters::prelude::*;
use plotters_svg::SVGBackend;
use serde::{Deserialize, Serialize};

use super::color::{ChartColor, DEFAULT_PALETTE};
use super::ChartError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarChart {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub categories: Vec<String>,
    pub series: Vec<BarSeries>,
    pub width: u32,
    pub height: u32,
    pub y_range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarSeries {
    pub name: String,
    /// One value per category.
    pub values: Vec<f64>,
    pub color: Option<ChartColor>,
}

pub(crate) fn render_bar(c: &BarChart) -> Result<String, ChartError> {
    let mut buf = String::new();
    {
        let root = SVGBackend::with_string(&mut buf, (c.width, c.height)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError(e.to_string()))?;

        let n_cats = c.categories.len();
        let (y_min, y_max) = c.y_range.unwrap_or_else(|| {
            let max = c
                .series
                .iter()
                .flat_map(|s| s.values.iter())
                .cloned()
                .fold(0.0_f64, f64::max);
            (0.0, max * 1.1)
        });

        let mut chart = ChartBuilder::on(&root)
            .caption(&c.title, ("sans-serif", 18u32).into_font())
            .margin(15u32)
            .x_label_area_size(45u32)
            .y_label_area_size(55u32)
            .build_cartesian_2d(0usize..n_cats, y_min..y_max)
            .map_err(|e| ChartError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&c.x_label)
            .y_desc(&c.y_label)
            .x_label_formatter(&|idx| {
                c.categories.get(*idx).cloned().unwrap_or_default()
            })
            .draw()
            .map_err(|e| ChartError(e.to_string()))?;

        let n_series = c.series.len().max(1);
        for (si, series) in c.series.iter().enumerate() {
            let color: RGBColor = series
                .color
                .unwrap_or(DEFAULT_PALETTE[si % DEFAULT_PALETTE.len()])
                .into();
            let bar_width = 0.7 / n_series as f64;
            let offset = si as f64 * bar_width - 0.35;
            let vals: Vec<(usize, f64)> = series
                .values
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v))
                .collect();
            let name = series.name.clone();
            chart
                .draw_series(vals.iter().map(|&(i, v)| {
                    let x0 = (i as f64 + offset) as usize;
                    let x1 = (i as f64 + offset + bar_width) as usize;
                    Rectangle::new([(x0, 0.0), (x1, v)], color.filled())
                }))
                .map_err(|e| ChartError(e.to_string()))?
                .label(name)
                .legend(move |(x, y)| {
                    Rectangle::new([(x, y - 5), (x + 15, y + 5)], color.filled())
                });
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
