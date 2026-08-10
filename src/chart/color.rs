use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChartColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ChartColor {
    pub const BLUE: Self = Self { r: 63, g: 127, b: 191 };
    pub const RED: Self = Self { r: 204, g: 51, b: 51 };
    pub const GREEN: Self = Self { r: 51, g: 153, b: 51 };
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const ORANGE: Self = Self { r: 230, g: 120, b: 30 };
}

impl From<ChartColor> for plotters::style::RGBColor {
    fn from(c: ChartColor) -> Self {
        plotters::style::RGBColor(c.r, c.g, c.b)
    }
}

pub(crate) const DEFAULT_PALETTE: [ChartColor; 6] = [
    ChartColor::BLUE,
    ChartColor::RED,
    ChartColor::GREEN,
    ChartColor::ORANGE,
    ChartColor::GRAY,
    ChartColor::BLACK,
];
