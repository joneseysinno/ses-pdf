use crate::element::Figure;
use super::escape::escape_text;

pub(crate) fn generate_svg_figure(path: &str, caption: &str) -> String {
    let escaped_cap = escape_text(caption);
    format!(
        "\n#figure(\n  image(\"{path}\"),\n  caption: [{escaped_cap}],\n)\n"
    )
}

pub(crate) fn generate_static_figure(path: &str, fig: &Figure) -> String {
    let escaped_cap = escape_text(&fig.caption);
    let pct = fig.width_pct;
    format!(
        "\n#figure(\n  image(\"{path}\", width: {pct}%),\n  caption: [{escaped_cap}],\n)\n"
    )
}
