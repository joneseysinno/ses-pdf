use crate::report::ReportMetadata;
use super::escape::escape_text;

pub(crate) fn generate_page_setup(meta: &ReportMetadata) -> String {
    let project = escape_text(&meta.project_name);
    let number = escape_text(&meta.project_number);
    let prepared_by = escape_text(&meta.prepared_by);
    let date = escape_text(&meta.date);
    let paper = meta.paper.typst_name();

    format!(
        r#"#set text(font: "New Computer Modern", size: 10pt, lang: "en")
#set par(leading: 0.65em, justify: true)
#set heading(numbering: "1.1")
#show heading: it => block(above: 1.2em, below: 0.8em, it)
#show figure: it => block(above: 1em, below: 1em, it)

#set page(
  paper: "{paper}",
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
