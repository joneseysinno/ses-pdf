/// Escape Typst special characters in plain text content.
/// Math strings must NOT be passed through this — they are placed inside `$...$` verbatim.
pub(crate) fn escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '#' | '@' | '_' | '*' | '[' | ']' | '<' | '>' | '\\' | '~' | '`' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}
