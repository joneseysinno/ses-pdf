use crate::element::HeadingLevel;
use super::escape::escape_text;

pub(crate) fn generate_heading(text: &str, level: HeadingLevel) -> String {
    let prefix = level.typst_prefix();
    let escaped = escape_text(text);
    format!("\n{prefix} {escaped}\n")
}
