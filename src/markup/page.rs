use crate::report::{ReportMetadata, TitleBlock};
use super::escape::escape_text;
use super::title_block::generate_title_block_footer;

pub(crate) fn generate_page_setup(
    meta: &ReportMetadata,
    title_block: Option<&TitleBlock>,
) -> String {
    if let Some(tb) = title_block {
        generate_drawing_page_setup(meta, tb)
    } else {
        generate_report_page_setup(meta)
    }
}

fn generate_report_page_setup(meta: &ReportMetadata) -> String {
    let project = escape_text(&meta.project_name);
    let number = escape_text(&meta.project_number);
    let prepared_by = escape_text(&meta.prepared_by);
    let date = escape_text(&meta.date);
    let paper = meta.paper.typst_name();
    let flipped = meta.orientation.flipped();

    format!(
        r#"#set text(font: "New Computer Modern", size: 10pt, lang: "en")
#set par(leading: 0.65em, justify: true)
#set heading(numbering: "1.1")
#show heading: it => block(above: 1.2em, below: 0.8em, it)
#show figure: it => block(above: 1em, below: 1em, it)

#set page(
  paper: "{paper}",
  flipped: {flipped},
  margin: (top: 1in, bottom: 1in, left: 1.25in, right: 1in),
  header: context {{
    if counter(page).get().first() > 1 {{
      grid(
        columns: (1fr, 1fr),
        align: (left + horizon, right + horizon),
        [*SES \| {project} \| {number}*],
        [Page #counter(page).display("1 of 1", both: true)],
      )
      line(length: 100%, stroke: 0.5pt + gray)
    }}
  }},
  footer: context {{
    if counter(page).get().first() > 1 {{
      line(length: 100%, stroke: 0.5pt + gray)
      grid(
        columns: (1fr, 1fr),
        align: (left + horizon, right + horizon),
        [Prepared by: {prepared_by} | {date}],
        text(style: "italic")[Preliminary — For Review Only],
      )
    }}
  }},
)

"#
    )
}

fn generate_drawing_page_setup(meta: &ReportMetadata, tb: &TitleBlock) -> String {
    let paper = meta.paper.typst_name();
    let flipped = meta.orientation.flipped();
    let footer = generate_title_block_footer(meta, tb);

    format!(
        r#"#set text(font: "New Computer Modern", size: 9pt, lang: "en")
#set par(leading: 0.55em, justify: false)
#set heading(numbering: none)
#show heading: it => block(above: 0.6em, below: 0.4em, {{
  set text(size: 11pt, weight: "bold")
  it.body
}})
#show figure: it => block(above: 0.4em, below: 0.4em, it)

#set page(
  paper: "{paper}",
  flipped: {flipped},
  margin: (top: 0.45in, bottom: 2.35in, left: 0.45in, right: 0.45in),
  foreground: {{
    place(
      top + left,
      dx: 0.2in,
      dy: 0.2in,
      rect(
        width: 100% - 0.4in,
        height: 100% - 0.4in,
        stroke: 1.25pt + black,
        fill: none,
      ),
    )
  }},
  footer-descent: 0.15in,
  footer: {{
    {footer}
  }},
)

"#
    )
}
