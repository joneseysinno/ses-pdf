//! Mock structural framing drawing sheet (Arch D landscape) with plan, schedules,
//! and title block.

use ses_pdf::*;

fn framing_plan_svg() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1100 720" width="1100" height="720">
  <rect width="1100" height="720" fill="#fafaf8"/>

  <!-- grid lines -->
  <g stroke="#c8c8c4" stroke-width="0.8" stroke-dasharray="6,4">
    <line x1="120" y1="80" x2="120" y2="580"/>
    <line x1="320" y1="80" x2="320" y2="580"/>
    <line x1="520" y1="80" x2="520" y2="580"/>
    <line x1="720" y1="80" x2="720" y2="580"/>
    <line x1="920" y1="80" x2="920" y2="580"/>
    <line x1="80" y1="120" x2="960" y2="120"/>
    <line x1="80" y1="320" x2="960" y2="320"/>
    <line x1="80" y1="520" x2="960" y2="520"/>
  </g>

  <!-- grid bubbles X -->
  <g font-family="sans-serif" font-size="14" text-anchor="middle">
    <circle cx="120" cy="50" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="120" y="55" fill="#1a1a1a">A</text>
    <circle cx="320" cy="50" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="320" y="55" fill="#1a1a1a">B</text>
    <circle cx="520" cy="50" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="520" y="55" fill="#1a1a1a">C</text>
    <circle cx="720" cy="50" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="720" y="55" fill="#1a1a1a">D</text>
    <circle cx="920" cy="50" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="920" y="55" fill="#1a1a1a">E</text>
  </g>

  <!-- grid bubbles Y -->
  <g font-family="sans-serif" font-size="14" text-anchor="middle">
    <circle cx="50" cy="120" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="50" y="125" fill="#1a1a1a">1</text>
    <circle cx="50" cy="320" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="50" y="325" fill="#1a1a1a">2</text>
    <circle cx="50" cy="520" r="16" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <text x="50" y="525" fill="#1a1a1a">3</text>
  </g>

  <!-- exterior walls -->
  <g fill="none" stroke="#2a2a2a" stroke-width="10" stroke-linejoin="miter">
    <rect x="120" y="120" width="800" height="400"/>
  </g>
  <!-- wall hatch (outer face) -->
  <g fill="none" stroke="#2a2a2a" stroke-width="3">
    <rect x="126" y="126" width="788" height="388"/>
  </g>

  <!-- interior wall (B to D at mid) -->
  <line x1="320" y1="320" x2="720" y2="320" stroke="#2a2a2a" stroke-width="8"/>
  <line x1="320" y1="316" x2="720" y2="316" stroke="#2a2a2a" stroke-width="2"/>

  <!-- interior partition A-B north bay -->
  <line x1="320" y1="120" x2="320" y2="320" stroke="#2a2a2a" stroke-width="6"/>

  <!-- window openings (gaps in exterior wall) + headers -->
  <!-- south wall windows -->
  <rect x="200" y="114" width="80" height="16" fill="#fafaf8" stroke="none"/>
  <line x1="200" y1="120" x2="280" y2="120" stroke="#4a7ab0" stroke-width="4"/>
  <text x="240" y="108" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#2c4c7c">WH-1</text>

  <rect x="420" y="114" width="100" height="16" fill="#fafaf8" stroke="none"/>
  <line x1="420" y1="120" x2="520" y2="120" stroke="#4a7ab0" stroke-width="4"/>
  <text x="470" y="108" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#2c4c7c">WH-2</text>

  <rect x="620" y="114" width="80" height="16" fill="#fafaf8" stroke="none"/>
  <line x1="620" y1="120" x2="700" y2="120" stroke="#4a7ab0" stroke-width="4"/>
  <text x="660" y="108" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#2c4c7c">WH-1</text>

  <!-- east wall windows -->
  <rect x="914" y="200" width="16" height="70" fill="#fafaf8" stroke="none"/>
  <line x1="920" y1="200" x2="920" y2="270" stroke="#4a7ab0" stroke-width="4"/>
  <text x="945" y="240" font-family="sans-serif" font-size="11" fill="#2c4c7c">WH-1</text>

  <rect x="914" y="380" width="16" height="70" fill="#fafaf8" stroke="none"/>
  <line x1="920" y1="380" x2="920" y2="450" stroke="#4a7ab0" stroke-width="4"/>
  <text x="945" y="420" font-family="sans-serif" font-size="11" fill="#2c4c7c">WH-1</text>

  <!-- beams (centerline symbols) -->
  <g stroke="#8b4513" stroke-width="2.5" fill="none">
    <!-- B-1 along grid 2, A to E -->
    <line x1="120" y1="320" x2="920" y2="320" stroke-dasharray="14,6"/>
    <!-- B-2 along grid B, 1 to 3 -->
    <line x1="320" y1="120" x2="320" y2="520" stroke-dasharray="14,6"/>
    <!-- B-2 along grid D -->
    <line x1="720" y1="120" x2="720" y2="520" stroke-dasharray="14,6"/>
  </g>
  <g font-family="sans-serif" font-size="12" fill="#8b4513">
    <text x="500" y="308">B-1</text>
    <text x="332" y="220">B-2</text>
    <text x="732" y="220">B-2</text>
  </g>

  <!-- columns -->
  <g fill="#1a1a1a" stroke="#1a1a1a" stroke-width="1">
    <rect x="112" y="112" width="16" height="16"/>
    <rect x="312" y="112" width="16" height="16"/>
    <rect x="512" y="112" width="16" height="16"/>
    <rect x="712" y="112" width="16" height="16"/>
    <rect x="912" y="112" width="16" height="16"/>

    <rect x="112" y="312" width="16" height="16"/>
    <rect x="312" y="312" width="16" height="16"/>
    <rect x="512" y="312" width="16" height="16"/>
    <rect x="712" y="312" width="16" height="16"/>
    <rect x="912" y="312" width="16" height="16"/>

    <rect x="112" y="512" width="16" height="16"/>
    <rect x="312" y="512" width="16" height="16"/>
    <rect x="512" y="512" width="16" height="16"/>
    <rect x="712" y="512" width="16" height="16"/>
    <rect x="912" y="512" width="16" height="16"/>
  </g>
  <g font-family="sans-serif" font-size="10" fill="#1a1a1a">
    <text x="132" y="108">C-1</text>
    <text x="532" y="108">C-1</text>
    <text x="132" y="308">C-2</text>
    <text x="532" y="308">C-2</text>
    <text x="132" y="548">C-1</text>
  </g>

  <!-- overall dimensions -->
  <g stroke="#1a1a1a" stroke-width="1" fill="#1a1a1a" font-family="sans-serif" font-size="12">
    <line x1="120" y1="600" x2="920" y2="600"/>
    <line x1="120" y1="594" x2="120" y2="606"/>
    <line x1="920" y1="594" x2="920" y2="606"/>
    <text x="520" y="620" text-anchor="middle">80'-0"</text>

    <line x1="980" y1="120" x2="980" y2="520"/>
    <line x1="974" y1="120" x2="986" y2="120"/>
    <line x1="974" y1="520" x2="986" y2="520"/>
    <text x="1005" y="330" text-anchor="middle" transform="rotate(90 1005 330)">40'-0"</text>

    <!-- bay dims -->
    <line x1="120" y1="640" x2="320" y2="640"/>
    <line x1="120" y1="634" x2="120" y2="646"/>
    <line x1="320" y1="634" x2="320" y2="646"/>
    <text x="220" y="658" text-anchor="middle">20'-0"</text>
  </g>

  <!-- north arrow -->
  <g transform="translate(1020, 80)">
    <circle cx="0" cy="0" r="28" fill="#fff" stroke="#1a1a1a" stroke-width="1.5"/>
    <polygon points="0,-20 6,8 0,4 -6,8" fill="#1a1a1a"/>
    <text x="0" y="24" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#1a1a1a">N</text>
  </g>

  <text x="550" y="700" font-family="sans-serif" font-size="14" text-anchor="middle" fill="#1a1a1a">
    PARTIAL FRAMING PLAN — LEVEL 1
  </text>
</svg>"##
}

#[test]
fn structural_framing_sheet_produces_valid_pdf() {
    let header_schedule = TableBuilder::new()
        .caption("Window Header Schedule")
        .column(ColumnSpec::auto().align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Left))
        .column(ColumnSpec::fractional(2).align(CellAlign::Left))
        .header(["Mark", "Opening", "Size", "Material", "Notes"])
        .row([
            CellValue::Text("WH-1".into()),
            CellValue::Text("4'-0\"".into()),
            CellValue::Text("(2) 2x10".into()),
            CellValue::Text("DF #2".into()),
            CellValue::Text("Typical exterior".into()),
        ])
        .row([
            CellValue::Text("WH-2".into()),
            CellValue::Text("5'-0\"".into()),
            CellValue::Text("(3) 2x10".into()),
            CellValue::Text("DF #2".into()),
            CellValue::Text("South wall, Grid B–C".into()),
        ])
        .build();

    let beam_schedule = TableBuilder::new()
        .caption("Beam Schedule")
        .column(ColumnSpec::auto().align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(2).align(CellAlign::Left))
        .header(["Mark", "Size", "Span", "Camber", "Notes"])
        .row([
            CellValue::Text("B-1".into()),
            CellValue::Text("W16×26".into()),
            CellValue::Text("20'-0\"".into()),
            CellValue::Text("—".into()),
            CellValue::Text("Grid 2, A–E continuous".into()),
        ])
        .row([
            CellValue::Text("B-2".into()),
            CellValue::Text("W12×19".into()),
            CellValue::Text("20'-0\"".into()),
            CellValue::Text("—".into()),
            CellValue::Text("Grids B and D, 1–3".into()),
        ])
        .build();

    let plan_fig = Figure {
        data: framing_plan_svg().as_bytes().to_vec(),
        format: ImageFormat::Svg,
        caption: "Partial framing plan — walls, window headers, beams, and columns".into(),
        width_pct: 95,
    };

    let bytes = Report::new(ReportMetadata {
        project_name: "Warehouse Addition — Partial Framing Plan".into(),
        project_number: "2026-S-042".into(),
        prepared_by: "SES Engineering".into(),
        checked_by: Some("PE Reviewer".into()),
        date: "August 27, 2026".into(),
        paper: PaperSize::ArchD,
        orientation: Orientation::Landscape,
    })
    .title_block(TitleBlock {
        sheet_title: "Foundation / Framing Plan".into(),
        sheet_number: "S-101".into(),
        scale: "1/4\" = 1'-0\"".into(),
        drawn_by: "AJ".into(),
        checked_by: Some("PE".into()),
        revisions: vec![
            ("A".into(), "2026-08-27".into(), "For Review".into()),
        ],
    })
    .section(
        Section::new("Level 1 Framing")
            .text(
                "Partial framing plan showing exterior and interior walls, window headers \
                 (WH), steel beams (B), and columns (C). See schedules for member sizes.",
            )
            .figure(plan_fig)
            .table(header_schedule)
            .table(beam_schedule),
    )
    .render()
    .expect("structural framing sheet render should succeed");

    assert!(!bytes.is_empty(), "PDF output must not be empty");
    assert!(
        bytes.starts_with(b"%PDF-"),
        "output must start with PDF header"
    );

    let _ = std::fs::create_dir_all("target");
    std::fs::write("target/structural_framing_sheet.pdf", &bytes)
        .expect("should be able to write test output");

    println!(
        "PDF written to target/structural_framing_sheet.pdf ({} bytes)",
        bytes.len()
    );
}
