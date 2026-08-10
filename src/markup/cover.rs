use crate::report::CoverPage;
use super::escape::escape_text;

pub(crate) fn generate_cover(cover: &CoverPage, has_logo: bool) -> String {
    let title = escape_text(&cover.title);
    let subtitle = cover.subtitle.as_deref().map(escape_text);

    let mut out = String::new();
    out.push_str("\n#page(header: none, footer: none)[\n");
    out.push_str("  #align(center)[\n");

    if has_logo {
        out.push_str(
            "    #image(\"logo.png\", width: 35%)\n    #v(1em)\n",
        );
    }

    out.push_str(&format!(
        "    #text(size: 26pt, weight: \"bold\")[{title}]\n"
    ));

    if let Some(sub) = subtitle {
        out.push_str(&format!("    #v(0.4em)\n    #text(size: 16pt, style: \"italic\")[{sub}]\n"));
    }

    out.push_str("    #v(1.5em)\n    #line(length: 60%, stroke: 1pt + black)\n    #v(1.5em)\n");

    // Project info table
    if !cover.project_info.is_empty() {
        out.push_str("    #table(\n      columns: (auto, 1fr),\n      stroke: none,\n      inset: 6pt,\n");
        for (k, v) in &cover.project_info {
            let k = escape_text(k);
            let v = escape_text(v);
            out.push_str(&format!("      [*{k}:*], [{v}],\n"));
        }
        out.push_str("    )\n");
    }

    // Revision table
    if !cover.revisions.is_empty() {
        out.push_str("    #v(2em)\n    #table(\n      columns: (auto, auto, 1fr, auto),\n      stroke: 0.5pt + gray,\n      inset: 6pt,\n");
        out.push_str("      [*Rev*], [*Date*], [*Description*], [*By*],\n");
        for (rev, date, desc, by) in &cover.revisions {
            let rev = escape_text(rev);
            let date = escape_text(date);
            let desc = escape_text(desc);
            let by = escape_text(by);
            out.push_str(&format!("      [{rev}], [{date}], [{desc}], [{by}],\n"));
        }
        out.push_str("    )\n");
    }

    out.push_str("  ]\n]\n#pagebreak()\n");
    out
}
