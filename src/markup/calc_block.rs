use crate::element::CalcBlock;
use super::escape::escape_text;

pub(crate) fn generate_calc_block(cb: &CalcBlock) -> String {
    let stroke_color = cb.status.stroke_color();
    let label = escape_text(&cb.label);

    let mut out = String::new();
    out.push_str(&format!(
        r#"
#block(
  width: 100%,
  stroke: (left: 3pt + rgb("{stroke_color}")),
  inset: (left: 12pt, top: 8pt, right: 8pt, bottom: 8pt),
  fill: rgb("f8f9fa"),
)[
  #grid(
    columns: (110pt, 1fr),
    align: (right + top, left + top),
    gutter: 6pt,
    [*Calculation:*], [*{label}*],
    [Formula:], [$ {formula} $],
    [Substitute:], [$ {sub} $],
    [Result:], [#text(weight: "bold")[$ {result} $]],
"#,
        formula = cb.formula,
        sub = cb.substitution,
        result = cb.result,
    ));

    if let Some(note) = &cb.note {
        let escaped_note = escape_text(note);
        out.push_str(&format!("    [Note:], [{escaped_note}],\n"));
    }

    out.push_str("  )\n]\n");
    out
}
