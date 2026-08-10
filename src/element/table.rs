use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub caption: Option<String>,
    pub columns: Vec<ColumnSpec>,
    pub header: Vec<String>,
    pub rows: Vec<Vec<CellValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSpec {
    pub width: ColumnWidth,
    pub align: CellAlign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnWidth {
    Auto,
    Fractional(u8),
    Points(f64),
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CellAlign {
    Left,
    #[default]
    Center,
    Right,
}

impl CellAlign {
    pub(crate) fn typst_name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellValue {
    Text(String),
    /// Typst math expression (without surrounding `$`).
    Math(String),
    Number {
        value: f64,
        decimals: u8,
        unit: Option<String>,
    },
    Bold(String),
    Empty,
}

impl ColumnSpec {
    pub fn auto() -> Self {
        Self {
            width: ColumnWidth::Auto,
            align: CellAlign::Center,
        }
    }

    pub fn fractional(n: u8) -> Self {
        Self {
            width: ColumnWidth::Fractional(n),
            align: CellAlign::Center,
        }
    }

    pub fn points(p: f64) -> Self {
        Self {
            width: ColumnWidth::Points(p),
            align: CellAlign::Center,
        }
    }

    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }
}

pub struct TableBuilder(Table);

impl TableBuilder {
    pub fn new() -> Self {
        Self(Table {
            caption: None,
            columns: Vec::new(),
            header: Vec::new(),
            rows: Vec::new(),
        })
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.0.caption = Some(caption.into());
        self
    }

    pub fn column(mut self, spec: ColumnSpec) -> Self {
        self.0.columns.push(spec);
        self
    }

    pub fn header(mut self, cells: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.0.header = cells.into_iter().map(Into::into).collect();
        self
    }

    pub fn row(mut self, cells: impl IntoIterator<Item = CellValue>) -> Self {
        self.0.rows.push(cells.into_iter().collect());
        self
    }

    pub fn build(self) -> Table {
        self.0
    }
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}
