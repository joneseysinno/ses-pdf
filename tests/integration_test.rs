use ses_pdf::*;

#[test]
fn minimal_report_produces_valid_pdf() {
    let bytes = Report::new(ReportMetadata {
        project_name: "Smoke Test".into(),
        project_number: "0000".into(),
        prepared_by: "Test".into(),
        checked_by: None,
        date: "January 1, 2025".into(),
        paper: PaperSize::Letter,
        orientation: Orientation::Portrait,
    })
    .section(
        Section::new("Introduction")
            .text("This is a minimal test report with inline math.")
            .equation(Equation::display("V_n = A_\"cv\" (alpha_c lambda sqrt(f'_c) + rho_t f_y)")),
    )
    .render()
    .expect("minimal render should succeed");

    assert!(!bytes.is_empty(), "PDF output must not be empty");
    assert!(bytes.starts_with(b"%PDF-"), "output must start with PDF header");
}

#[test]
fn full_engineering_report_produces_valid_pdf() {
    let material_table = TableBuilder::new()
        .caption("Material Properties")
        .column(ColumnSpec::auto().align(CellAlign::Left))
        .column(ColumnSpec::fractional(1).align(CellAlign::Center))
        .column(ColumnSpec::fractional(1).align(CellAlign::Right))
        .header(["Property", "Symbol", "Value"])
        .row([
            CellValue::Text("Concrete Compressive Strength".into()),
            CellValue::Math("f'_c".into()),
            CellValue::Text("4000 psi".into()),
        ])
        .row([
            CellValue::Text("Steel Yield Strength".into()),
            CellValue::Math("f_y".into()),
            CellValue::Text("60 ksi".into()),
        ])
        .row([
            CellValue::Text("Plate Thickness".into()),
            CellValue::Math("t_w".into()),
            CellValue::Number { value: 12.0, decimals: 0, unit: Some("in".into()) },
        ])
        .build();

    let moment_block = CalcBlock::new("Nominal Moment Capacity")
        .formula("M_n = A_s f_y (d - a/2)")
        .substitution("M_n = (0.60 \"in\"^2)(60 \"ksi\")(14.5 - 1.25 \"in\")")
        .result("M_n = 477 \"kip-in\" = 39.8 \"kip-ft\"")
        .status(CalcStatus::Governs)
        .build()
        .expect("calc block builder should not fail");

    let shear_block = CalcBlock::new("Shear Capacity Check")
        .formula("phi V_n >= V_u")
        .substitution("phi V_n = 0.75 times 2 times 1.0 times sqrt(4000) times 12 times 18")
        .result("phi V_n = 102 \"kips\" > V_u = 85 \"kips\" checkmark")
        .status(CalcStatus::Ok)
        .build()
        .expect("shear block should build");

    let load_chart = Chart::Line(LineChart {
        title: "Load vs. Midspan Deflection".into(),
        x_label: "Uniform Load w (kip/ft)".into(),
        y_label: "Deflection delta (in)".into(),
        series: vec![
            LineSeries {
                name: "Dead Load".into(),
                points: vec![(0.0, 0.0), (0.5, 0.05), (1.0, 0.10), (2.0, 0.41), (3.0, 0.92)],
                color: Some(ChartColor::BLUE),
            },
            LineSeries {
                name: "Dead + Live".into(),
                points: vec![(0.0, 0.0), (0.5, 0.08), (1.0, 0.15), (2.0, 0.62), (3.0, 1.40)],
                color: Some(ChartColor::RED),
            },
        ],
        width: 800,
        height: 500,
        x_range: None,
        y_range: None,
    });

    let pm_diagram = Chart::PmInteraction(PmInteractionDiagram {
        title: "P-M Interaction — Column 12x20".into(),
        envelope: vec![
            (0.0, 800.0),
            (120.0, 750.0),
            (200.0, 600.0),
            (230.0, 400.0),
            (180.0, 0.0),
            (0.0, -100.0),
            (0.0, 800.0),
        ],
        demand_points: vec![
            DemandPoint { m: 150.0, p: 450.0, label: "1.2D+1.6L".into() },
            DemandPoint { m: 90.0, p: 300.0, label: "0.9D+1.0W".into() },
        ],
        m_label: "M (kip-ft)".into(),
        p_label: "P (kips)".into(),
        width: 700,
        height: 600,
    });

    let scatter_chart = Chart::Scatter(ScatterChart {
        title: "Measured vs. Predicted Shear Strength".into(),
        x_label: "Predicted Vn (kips)".into(),
        y_label: "Measured Vn (kips)".into(),
        series: vec![ScatterSeries {
            name: "Test data".into(),
            points: vec![(50.0, 52.0), (80.0, 78.0), (100.0, 103.0), (130.0, 128.0)],
            color: Some(ChartColor::BLUE),
        }],
        width: 600,
        height: 500,
        x_range: Some((0.0, 150.0)),
        y_range: Some((0.0, 150.0)),
    });

    let bytes = Report::new(ReportMetadata {
        project_name: "Typical Beam — Grid B5".into(),
        project_number: "2025-001".into(),
        prepared_by: "SES Engineering".into(),
        checked_by: Some("PE Reviewer".into()),
        date: "March 14, 2025".into(),
        paper: PaperSize::Letter,
        orientation: Orientation::Portrait,
    })
    .cover(CoverPage {
        title: "Structural Beam Design".into(),
        subtitle: Some("Gravity Load Analysis".into()),
        logo_png: None,
        project_info: vec![
            ("Client".into(), "Example Corp.".into()),
            ("Location".into(), "Building A, Level 3".into()),
        ],
        revisions: vec![
            ("A".into(), "2025-03-14".into(), "For Review".into(), "AJ".into()),
        ],
    })
    .section(
        Section::new("Material Properties")
            .text("The following material properties are used for all design calculations.")
            .table(material_table),
    )
    .section(
        Section::new("Flexural Design")
            .text("The nominal moment capacity is computed per ACI 318-19 Section 22.2.")
            .equation(
                Equation::display("phi M_n = phi dot A_s dot f_y dot (d - a/2)")
                    .with_label("eq:moment"),
            )
            .calc(moment_block),
    )
    .section(
        Section::new("Shear Design")
            .text("Shear capacity per ACI 318-19 Section 22.5.")
            .equation(Equation::display("V_n = 2 lambda sqrt(f'_c) b_w d"))
            .calc(shear_block),
    )
    .section(
        Section::new("Deflection Analysis")
            .text("The following chart shows midspan deflection as a function of applied uniform load.")
            .chart(load_chart, "Midspan Deflection vs. Applied Load"),
    )
    .section(
        Section::new("Column Capacity Check")
            .text("P-M interaction diagram for column 12x20 under combined axial and bending.")
            .chart(pm_diagram, "P-M Interaction Diagram — Column 12x20"),
    )
    .section(
        Section::new("Verification Data")
            .text("Comparison of predicted vs. measured shear strength from test database.")
            .chart(scatter_chart, "Predicted vs. Measured Shear Strength"),
    )
    .render()
    .expect("full report render should succeed");

    assert!(!bytes.is_empty(), "PDF output must not be empty");
    assert!(bytes.starts_with(b"%PDF-"), "output must start with PDF header");

    // Write for manual inspection
    let _ = std::fs::create_dir_all("target");
    std::fs::write("target/test_report.pdf", &bytes)
        .expect("should be able to write test output");

    println!("PDF written to target/test_report.pdf ({} bytes)", bytes.len());
}
