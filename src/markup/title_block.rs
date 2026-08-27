use crate::report::{ReportMetadata, TitleBlock};
use super::escape::escape_text;

/// Emit Typst markup for the drawing-sheet title block footer content.
pub(crate) fn generate_title_block_footer(
    meta: &ReportMetadata,
    tb: &TitleBlock,
) -> String {
    let project = escape_text(&meta.project_name);
    let number = escape_text(&meta.project_number);
    let prepared_by = escape_text(&meta.prepared_by);
    let date = escape_text(&meta.date);
    let sheet_title = escape_text(&tb.sheet_title);
    let sheet_number = escape_text(&tb.sheet_number);
    let scale = escape_text(&tb.scale);
    let drawn_by = escape_text(&tb.drawn_by);
    let checked = tb
        .checked_by
        .as_ref()
        .map(|s| escape_text(s))
        .unwrap_or_else(|| "—".into());

    let mut rev_rows = String::new();
    if tb.revisions.is_empty() {
        rev_rows.push_str("[—], [—], [—],\n");
    } else {
        for (rev, rev_date, desc) in &tb.revisions {
            rev_rows.push_str(&format!(
                "[{}], [{}], [{}],\n",
                escape_text(rev),
                escape_text(rev_date),
                escape_text(desc),
            ));
        }
    }

    format!(
        r#"block(
  width: 100%,
  stroke: 1pt + black,
  inset: 0pt,
  grid(
    columns: (1.6fr, 2.2fr, 1.1fr),
    rows: auto,
    stroke: 0.5pt + black,
    inset: 6pt,
    align: (left + top, left + top, left + top),
    // Firm / project
    [
      #set text(size: 8pt)
      *SES Engineering*\
      {project}\
      Proj. {number}\
      {date}
    ],
    // Sheet title + scale / drawn
    [
      #set text(size: 11pt)
      *{sheet_title}*
      #v(4pt)
      #set text(size: 8pt)
      Scale: {scale}\
      Drawn: {drawn_by}  ·  Checked: {checked}\
      Prepared: {prepared_by}
    ],
    // Sheet number + revisions
    [
      #set text(size: 8pt)
      Sheet\
      #set text(size: 18pt, weight: "bold")
      {sheet_number}
      #v(4pt)
      #set text(size: 7pt, weight: "regular")
      *Revisions*
      #table(
        columns: (auto, auto, 1fr),
        stroke: 0.4pt + gray,
        inset: 3pt,
        [*Rev*], [*Date*], [*Description*],
        {rev_rows}
      )
    ],
  ),
)"#
    )
}
