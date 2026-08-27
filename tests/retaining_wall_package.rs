//! Mock calculation package for a cantilever concrete retaining wall (CRW-1).

use ses_pdf::*;

fn wall_elevation_svg() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 420" width="520" height="420">
  <rect width="520" height="420" fill="#f7f7f5"/>
  <!-- retained soil hatch (heel side) -->
  <defs>
    <pattern id="soil" patternUnits="userSpaceOnUse" width="8" height="8">
      <path d="M0,8 L8,0" stroke="#c4b89a" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="230" y="40" width="200" height="280" fill="url(#soil)" opacity="0.85"/>
  <rect x="230" y="40" width="200" height="280" fill="none" stroke="#a09070" stroke-width="1"/>
  <!-- footing -->
  <rect x="80" y="300" width="280" height="40" fill="#8a8a8a" stroke="#333" stroke-width="1.5"/>
  <!-- stem -->
  <rect x="200" y="60" width="30" height="240" fill="#9a9a9a" stroke="#333" stroke-width="1.5"/>
  <!-- grade line retained -->
  <line x1="230" y1="60" x2="430" y2="60" stroke="#5a7a4a" stroke-width="2"/>
  <!-- grade line front -->
  <line x1="40" y1="300" x2="200" y2="300" stroke="#5a7a4a" stroke-width="2"/>
  <!-- dimension H -->
  <line x1="60" y1="60" x2="60" y2="300" stroke="#1a1a1a" stroke-width="1"/>
  <line x1="55" y1="60" x2="65" y2="60" stroke="#1a1a1a" stroke-width="1"/>
  <line x1="55" y1="300" x2="65" y2="300" stroke="#1a1a1a" stroke-width="1"/>
  <text x="42" y="185" font-family="sans-serif" font-size="14" fill="#1a1a1a">H</text>
  <!-- dimension B -->
  <line x1="80" y1="360" x2="360" y2="360" stroke="#1a1a1a" stroke-width="1"/>
  <line x1="80" y1="355" x2="80" y2="365" stroke="#1a1a1a" stroke-width="1"/>
  <line x1="360" y1="355" x2="360" y2="365" stroke="#1a1a1a" stroke-width="1"/>
  <text x="210" y="380" font-family="sans-serif" font-size="14" fill="#1a1a1a">B</text>
  <!-- stem thickness t -->
  <line x1="200" y1="40" x2="230" y2="40" stroke="#1a1a1a" stroke-width="1"/>
  <text x="205" y="32" font-family="sans-serif" font-size="12" fill="#1a1a1a">t</text>
  <!-- toe / heel labels -->
  <text x="100" y="295" font-family="sans-serif" font-size="11" fill="#333">toe</text>
  <text x="280" y="295" font-family="sans-serif" font-size="11" fill="#333">heel</text>
  <text x="250" y="120" font-family="sans-serif" font-size="12" fill="#6a5a3a">retained fill</text>
  <text x="140" y="400" font-family="sans-serif" font-size="13" fill="#1a1a1a">Wall Elevation (typ.)</text>
</svg>"##
}

fn loading_diagram_svg() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 420" width="520" height="420">
  <rect width="520" height="420" fill="#f7f7f5"/>
  <!-- footing -->
  <rect x="80" y="300" width="280" height="40" fill="#8a8a8a" stroke="#333" stroke-width="1.5"/>
  <!-- stem -->
  <rect x="200" y="60" width="30" height="240" fill="#9a9a9a" stroke="#333" stroke-width="1.5"/>
  <!-- surcharge qs strip -->
  <rect x="230" y="40" width="160" height="18" fill="#d08040" stroke="#8a5020" stroke-width="1"/>
  <text x="280" y="53" font-family="sans-serif" font-size="11" fill="#fff">q_s</text>
  <!-- triangular active earth pressure -->
  <polygon points="230,60 320,300 230,300" fill="#4a7ab0" fill-opacity="0.35" stroke="#2c4c7c" stroke-width="1.5"/>
  <!-- Pa resultant arrow -->
  <line x1="275" y1="220" x2="200" y2="220" stroke="#1a3a6a" stroke-width="2.5" marker-end="url(#arrow)"/>
  <defs>
    <marker id="arrow" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#1a3a6a"/>
    </marker>
  </defs>
  <text x="285" y="215" font-family="sans-serif" font-size="13" fill="#1a3a6a">P_a</text>
  <!-- H/3 label -->
  <line x1="340" y1="220" x2="340" y2="300" stroke="#1a1a1a" stroke-width="1" stroke-dasharray="3,2"/>
  <text x="348" y="265" font-family="sans-serif" font-size="12" fill="#1a1a1a">H/3</text>
  <!-- self-weight arrows -->
  <line x1="215" y1="140" x2="215" y2="180" stroke="#555" stroke-width="1.5"/>
  <polygon points="210,180 215,190 220,180" fill="#555"/>
  <text x="145" y="165" font-family="sans-serif" font-size="11" fill="#555">W_stem</text>
  <line x1="220" y1="320" x2="220" y2="350" stroke="#555" stroke-width="1.5"/>
  <polygon points="215,350 220,360 225,350" fill="#555"/>
  <text x="100" y="345" font-family="sans-serif" font-size="11" fill="#555">W_ftg</text>
  <text x="120" y="400" font-family="sans-serif" font-size="13" fill="#1a1a1a">Loading Diagram — Active Earth Pressure</text>
</svg>"##
}

#[test]
fn retaining_wall_calculation_package_produces_valid_pdf() {
    let geometry_table = TableBuilder::new()
        .caption("Wall Geometry")
        .column(ColumnSpec::auto().align(CellAlign::Left))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Right))
        .header(["Parameter", "Symbol", "Value"])
        .row([
            CellValue::Text("Retained height".into()),
            CellValue::Math("H".into()),
            CellValue::Text("12.0 ft".into()),
        ])
        .row([
            CellValue::Text("Stem thickness".into()),
            CellValue::Math("t".into()),
            CellValue::Text("12 in".into()),
        ])
        .row([
            CellValue::Text("Footing width".into()),
            CellValue::Math("B".into()),
            CellValue::Text("8.0 ft".into()),
        ])
        .row([
            CellValue::Text("Toe length".into()),
            CellValue::Math("L_\"toe\"".into()),
            CellValue::Text("2.5 ft".into()),
        ])
        .row([
            CellValue::Text("Heel length".into()),
            CellValue::Math("L_\"heel\"".into()),
            CellValue::Text("4.5 ft".into()),
        ])
        .row([
            CellValue::Text("Footing thickness".into()),
            CellValue::Math("t_f".into()),
            CellValue::Text("18 in".into()),
        ])
        .build();

    let soil_table = TableBuilder::new()
        .caption("Soil and Load Assumptions")
        .column(ColumnSpec::auto().align(CellAlign::Left))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Right))
        .header(["Parameter", "Symbol", "Value"])
        .row([
            CellValue::Text("Backfill unit weight".into()),
            CellValue::Math("gamma".into()),
            CellValue::Text("120 pcf".into()),
        ])
        .row([
            CellValue::Text("Friction angle".into()),
            CellValue::Math("phi".into()),
            CellValue::Text("30 deg".into()),
        ])
        .row([
            CellValue::Text("Active earth pressure coeff.".into()),
            CellValue::Math("K_a".into()),
            CellValue::Number {
                value: 0.333,
                decimals: 3,
                unit: None,
            },
        ])
        .row([
            CellValue::Text("Uniform surcharge".into()),
            CellValue::Math("q_s".into()),
            CellValue::Text("100 psf".into()),
        ])
        .row([
            CellValue::Text("Base friction coeff.".into()),
            CellValue::Math("mu".into()),
            CellValue::Number {
                value: 0.45,
                decimals: 2,
                unit: None,
            },
        ])
        .row([
            CellValue::Text("Allowable bearing".into()),
            CellValue::Math("q_\"all\"".into()),
            CellValue::Text("3.0 ksf".into()),
        ])
        .build();

    let summary_table = TableBuilder::new()
        .caption("Design Check Summary")
        .column(ColumnSpec::auto().align(CellAlign::Left))
        .column(ColumnSpec::fractional(1).align(CellAlign::Right))
        .column(ColumnSpec::fractional(1).align(CellAlign::Right))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .header(["Check", "Demand / FS", "Limit", "Result"])
        .row([
            CellValue::Text("Overturning".into()),
            CellValue::Text("FS = 2.15".into()),
            CellValue::Text("1.50 min".into()),
            CellValue::Text("PASS".into()),
        ])
        .row([
            CellValue::Text("Sliding".into()),
            CellValue::Text("FS = 1.72".into()),
            CellValue::Text("1.50 min".into()),
            CellValue::Text("PASS".into()),
        ])
        .row([
            CellValue::Text("Bearing pressure".into()),
            CellValue::Text("q_max = 2.41 ksf".into()),
            CellValue::Text("3.00 ksf".into()),
            CellValue::Text("PASS".into()),
        ])
        .row([
            CellValue::Text("Stem flexure".into()),
            CellValue::Text("M_u = 18.4 kip-ft".into()),
            CellValue::Text("phi M_n = 24.6 kip-ft".into()),
            CellValue::Text("PASS".into()),
        ])
        .build();

    let overturning = CalcBlock::new("Overturning Stability")
        .formula("\"FS\"_\"OT\" = M_R / M_O")
        .substitution("\"FS\"_\"OT\" = 52.8 \"kip-ft\" / 24.6 \"kip-ft\"")
        .result("\"FS\"_\"OT\" = 2.15 > 1.50  \"PASS\"")
        .note("Resisting moment from wall and footing self-weight plus heel soil; overturning from active thrust at H/3.")
        .status(CalcStatus::Ok)
        .build()
        .expect("overturning block should build");

    let sliding = CalcBlock::new("Sliding Stability")
        .formula("\"FS\"_\"SL\" = (mu sum V) / P_a")
        .substitution("\"FS\"_\"SL\" = (0.45 times 18.4 \"kips\") / 4.80 \"kips\"")
        .result("\"FS\"_\"SL\" = 1.72 > 1.50  \"PASS\"")
        .status(CalcStatus::Ok)
        .build()
        .expect("sliding block should build");

    let bearing = CalcBlock::new("Bearing Pressure")
        .formula("q_\"max\" = (sum V)/B (1 + 6 e / B)")
        .substitution("q_\"max\" = 18.4 / 8.0 (1 + 6 times 0.85 / 8.0)")
        .result("q_\"max\" = 2.41 \"ksf\" < q_\"all\" = 3.0 \"ksf\"  \"PASS\"")
        .status(CalcStatus::Ok)
        .build()
        .expect("bearing block should build");

    let stem_flexure = CalcBlock::new("Stem Flexural Capacity")
        .formula("phi M_n >= M_u")
        .substitution("phi M_n = 0.90 times A_s f_y (d - a/2) = 24.6 \"kip-ft\"")
        .result("phi M_n = 24.6 \"kip-ft\" > M_u = 18.4 \"kip-ft\"  \"PASS\"")
        .note("Stem reinforced with #5 @ 12 in. o.c. vertical bars, Grade 60.")
        .status(CalcStatus::Ok)
        .build()
        .expect("stem flexure block should build");

    let wall_fig = Figure {
        data: wall_elevation_svg().as_bytes().to_vec(),
        format: ImageFormat::Svg,
        caption: "Cantilever retaining wall elevation — CRW-1".into(),
        width_pct: 80,
    };

    let loading_fig = Figure {
        data: loading_diagram_svg().as_bytes().to_vec(),
        format: ImageFormat::Svg,
        caption: "Active earth pressure and surcharge loading diagram".into(),
        width_pct: 80,
    };

    let bytes = Report::new(ReportMetadata {
        project_name: "CRW-1 Cantilever Retaining Wall".into(),
        project_number: "2025-RW-014".into(),
        prepared_by: "SES Engineering".into(),
        checked_by: Some("PE Reviewer".into()),
        date: "August 27, 2026".into(),
        paper: PaperSize::Letter,
    })
    .cover(CoverPage {
        title: "Concrete Retaining Wall".into(),
        subtitle: Some("Calculation Package — Stability and Strength".into()),
        logo_png: None,
        project_info: vec![
            ("Client".into(), "Site Development LLC".into()),
            ("Location".into(), "Parcel B, Lot 12".into()),
            ("Wall ID".into(), "CRW-1".into()),
        ],
        revisions: vec![(
            "A".into(),
            "2026-08-27".into(),
            "For Review".into(),
            "AJ".into(),
        )],
    })
    .section(
        Section::new("Geometry")
            .text(
                "Cantilever concrete retaining wall CRW-1 retains 12 ft of granular backfill. \
                 Stem and footing are cast-in-place normalweight concrete, f'_c = 4000 psi.",
            )
            .table(geometry_table)
            .figure(wall_fig),
    )
    .section(
        Section::new("Loading")
            .text(
                "Lateral earth pressure is computed using Rankine active theory with a level \
                 backfill and a uniform surcharge q_s representing temporary construction loading.",
            )
            .table(soil_table)
            .figure(loading_fig),
    )
    .section(
        Section::new("Analysis Equations")
            .text("The following equations govern global stability and stem design.")
            .equation(
                Equation::display("K_a = (1 - sin(phi)) / (1 + sin(phi))")
                    .with_label("eq:ka"),
            )
            .equation(
                Equation::display("P_a = 1/2 K_a gamma H^2 + K_a q_s H")
                    .with_label("eq:pa"),
            )
            .equation(
                Equation::display("\"FS\"_\"OT\" = M_R / M_O >= 1.50")
                    .with_label("eq:ot"),
            )
            .equation(
                Equation::display("\"FS\"_\"SL\" = (mu sum V) / P_a >= 1.50")
                    .with_label("eq:sl"),
            )
            .equation(
                Equation::display("q_\"max\" = (sum V)/B (1 + 6 e / B) <= q_\"all\"")
                    .with_label("eq:bearing"),
            )
            .equation(
                Equation::display("phi M_n = phi A_s f_y (d - a/2) >= M_u")
                    .with_label("eq:stem"),
            ),
    )
    .section(
        Section::new("Stability and Strength Checks")
            .text(
                "Results below compare demand to code minimum factors of safety and member \
                 capacity. All governing checks pass for the stated geometry and loads.",
            )
            .calc(overturning)
            .calc(sliding)
            .calc(bearing)
            .calc(stem_flexure)
            .table(summary_table),
    )
    .render()
    .expect("retaining wall package render should succeed");

    assert!(!bytes.is_empty(), "PDF output must not be empty");
    assert!(
        bytes.starts_with(b"%PDF-"),
        "output must start with PDF header"
    );

    let _ = std::fs::create_dir_all("target");
    std::fs::write("target/retaining_wall_package.pdf", &bytes)
        .expect("should be able to write test output");

    println!(
        "PDF written to target/retaining_wall_package.pdf ({} bytes)",
        bytes.len()
    );
}
