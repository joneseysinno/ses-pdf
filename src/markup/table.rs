use crate::element::{CellValue, ColumnWidth, Table};
use super::escape::escape_text;

pub(crate) fn generate_table(t: &Table) -> String {
    let col_spec: Vec<String> = if t.columns.is_empty() {
        let n = t.header.len().max(1);
        vec!["1fr".to_string(); n]
    } else {
        t.columns
            .iter()
            .map(|c| match &c.width {
                ColumnWidth::Auto => "auto".to_string(),
                ColumnWidth::Fractional(n) => format!("{n}fr"),
                ColumnWidth::Points(p) => format!("{p}pt"),
            })
            .collect()
    };

    let align_spec: Vec<&str> = if t.columns.is_empty() {
        vec!["center"; t.header.len().max(1)]
    } else {
        t.columns.iter().map(|c| c.align.typst_name()).collect()
    };

    let cols_str = col_spec.join(", ");
    let align_str = align_spec.join(", ");

    let mut out = String::new();
    out.push_str("\n#figure(\n  table(\n");
    out.push_str(&format!("    columns: ({cols_str}),\n"));
    out.push_str(&format!("    align: ({align_str}),\n"));
    out.push_str(
        "    stroke: (x: none, y: 0.5pt + gray),\n    inset: (x: 8pt, y: 5pt),\n",
    );
    out.push_str(
        "    fill: (_, row) => if row == 0 { rgb(\"2c4c7c\") } else if calc.even(row) { rgb(\"f0f4f8\") } else { none },\n",
    );

    // Header row
    out.push_str("    table.header(\n");
    for h in &t.header {
        let escaped = escape_text(h);
        out.push_str(&format!(
            "      table.cell()[#text(fill: white, weight: \"bold\")[{escaped}]],\n"
        ));
    }
    out.push_str("    ),\n");

    // Data rows
    for row in &t.rows {
        for cell in row {
            let cell_str = render_cell(cell);
            out.push_str(&format!("    {cell_str},\n"));
        }
    }

    out.push_str("  ),\n");

    if let Some(cap) = &t.caption {
        let escaped_cap = escape_text(cap);
        out.push_str(&format!("  caption: [{escaped_cap}],\n"));
    }
    out.push_str(")\n");
    out
}

fn render_cell(cell: &CellValue) -> String {
    match cell {
        CellValue::Text(s) => format!("[{}]", escape_text(s)),
        CellValue::Math(m) => format!("[${}$]", m),
        CellValue::Number { value, decimals, unit } => {
            let formatted = format!("{:.prec$}", value, prec = *decimals as usize);
            // Use as_deref() to avoid Rust 2024 match ergonomics issue with Option<String>
            match unit.as_deref() {
                Some(u) => format!("[{} {}]", formatted, escape_text(u)),
                None => format!("[{}]", formatted),
            }
        }
        CellValue::Bold(s) => format!("[*{}*]", escape_text(s)),
        CellValue::Empty => "[]".to_string(),
    }
}
