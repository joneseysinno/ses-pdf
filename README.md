# ses-pdf

Professional PDF report engine for structural engineering calculations, written in Rust.

Compiles a structured document model into a publication-quality PDF using the
[Typst](https://typst.app) typesetting engine under the hood. Reports can contain:

- **Equations** — Typst math notation, inline or display
- **Calculation blocks** — formula → substitution → numeric result, with status indicators
- **Tables** — shaded alternating rows and a styled header
- **Charts** — line, bar, scatter, and P-M interaction diagrams (vector SVG)
- **Figures** — user-supplied PNG, JPEG, or SVG images
- **Cover page** — title, logo, project info table, revision history
- **Professional layout** — page numbers, header/footer, section numbering; Letter/Tabloid/A4 and Arch A–E / E1
- **Drawing sheets** — landscape Arch sizes with sheet border and title block (sheet number, scale, revisions)

## Installation

```toml
[dependencies]
ses-pdf = "0.2"
```

Requires Rust **1.92** or later.

## Quick start

```rust
use ses_pdf::*;

let pdf_bytes = Report::new(ReportMetadata {
    project_name:   "Grid B5 Beam".into(),
    project_number: "2025-001".into(),
    prepared_by:    "SES Engineering".into(),
    checked_by:     None,
    date:           "March 14, 2025".into(),
    paper:          PaperSize::Letter,
    orientation:    Orientation::Portrait,
})
.cover(CoverPage {
    title:        "Structural Beam Design".into(),
    subtitle:     Some("Gravity Load Analysis".into()),
    logo_png:     None,
    project_info: vec![("Client".into(), "Example Corp.".into())],
    revisions:    vec![("A".into(), "2025-03-14".into(), "For Review".into(), "AJ".into())],
})
.section(
    Section::new("Flexural Design")
        .text("Nominal moment capacity per ACI 318-19 Section 22.2.")
        .equation(Equation::display(
            "phi M_n = phi dot A_s dot f_y dot (d - a/2)"
        ))
        .calc(
            CalcBlock::new("Nominal Moment Capacity")
                .formula("M_n = A_s f_y (d - a/2)")
                .substitution("M_n = (0.60)(60)(14.5 - 1.25)")
                .result("M_n = 477 \"kip-in\" = 39.8 \"kip-ft\"")
                .status(CalcStatus::Governs)
                .build()?,
        ),
)
.section(
    Section::new("Load vs. Deflection")
        .chart(
            Chart::Line(LineChart {
                title:   "Midspan Deflection".into(),
                x_label: "Load w (kip/ft)".into(),
                y_label: "Deflection δ (in)".into(),
                series:  vec![LineSeries {
                    name:   "Dead + Live".into(),
                    points: vec![(0.0, 0.0), (1.0, 0.15), (2.0, 0.62), (3.0, 1.40)],
                    color:  Some(ChartColor::BLUE),
                }],
                width: 800, height: 500,
                x_range: None, y_range: None,
            }),
            "Midspan Deflection vs. Applied Load",
        ),
)
.render()?;

std::fs::write("report.pdf", pdf_bytes)?;
```

## Chart types

| Variant | Use case |
|---------|----------|
| `Chart::Line(LineChart)` | Load-deflection, time series, any x-y data |
| `Chart::Bar(BarChart)` | Comparison of quantities across categories |
| `Chart::Scatter(ScatterChart)` | Test data vs. prediction, correlation plots |
| `Chart::PmInteraction(PmInteractionDiagram)` | Column/wall P-M capacity envelope with demand points |

## Math notation

Equations use **Typst math syntax**, not LaTeX:

| LaTeX | Typst |
|-------|-------|
| `\phi` | `phi` |
| `\frac{a}{b}` | `a/b` |
| `\sqrt{x}` | `sqrt(x)` |
| `\times` | `times` |
| `A_{cv}` (multi-letter subscript) | `A_"cv"` |
| Display math `\[ ... \]` | `$ ... $` (spaces inside the `$`) |

Full reference: [typst.app/docs/reference/math](https://typst.app/docs/reference/math/)

## Calculation blocks

```rust
CalcBlock::new("Shear Capacity")
    .formula("phi V_n = phi dot 2 lambda sqrt(f'_c) b_w d")
    .substitution("= 0.75 dot 2 dot 1.0 dot sqrt(4000) dot 12 dot 18")
    .result("= 102 \"kips\"  >  V_u = 85 \"kips\"")
    .status(CalcStatus::Ok)   // Ok | Governs | DoesNotGovern
    .build()?
```

Status controls the left border color: blue (Ok), green (Governs), gray (DoesNotGovern).

## Drawing sheets

Use Arch paper sizes with landscape orientation and a `TitleBlock` for structural
drawing sheets. Attaching a title block switches page chrome to a sheet border and
footer title block (instead of calculation-report headers).

| `PaperSize` | Typst name | Size |
|-------------|------------|------|
| `ArchA` | `arch-a` | 9 × 12 in |
| `ArchB` | `arch-b` | 12 × 18 in |
| `ArchC` | `arch-c` | 18 × 24 in |
| `ArchD` | `arch-d` | 24 × 36 in |
| `ArchE` | `arch-e` | 36 × 48 in |
| `ArchE1` | `arch-e1` | 30 × 42 in |

```rust
Report::new(ReportMetadata {
    project_name: "Warehouse Addition".into(),
    project_number: "2026-S-042".into(),
    prepared_by: "SES Engineering".into(),
    checked_by: Some("PE Reviewer".into()),
    date: "August 27, 2026".into(),
    paper: PaperSize::ArchD,
    orientation: Orientation::Landscape,
})
.title_block(TitleBlock {
    sheet_title: "Partial Framing Plan".into(),
    sheet_number: "S-101".into(),
    scale: "1/4\" = 1'-0\"".into(),
    drawn_by: "AJ".into(),
    checked_by: Some("PE".into()),
    revisions: vec![("A".into(), "2026-08-27".into(), "For Review".into())],
})
/* .section(... plan figure, schedules ...) */
.render()?;
```

## License

MIT
