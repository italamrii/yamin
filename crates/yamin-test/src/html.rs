//! HTML reference page (D9-c): browsers implement BiDi and shaping, so each
//! case's logical input, placed in a block with its paragraph direction, shows
//! what the terminal should draw.

use crate::cases::{all, Category, Dir};

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn reference_page() -> String {
    let mut s = String::from(
        r#"<!doctype html>
<html lang="ar">
<head>
<meta charset="utf-8">
<title>yamin-test — reference rendering</title>
<style>
  body { font-family: system-ui, sans-serif; max-width: 56rem; margin: 2rem auto; padding: 0 1rem; line-height: 1.5; }
  h2 { border-bottom: 1px solid #8884; padding-bottom: .25rem; margin-top: 2.5rem; }
  section { margin: 1.25rem 0; }
  h3 { font-size: .9rem; font-family: ui-monospace, monospace; margin: 0; color: #666; }
  .line { font-family: "DejaVu Sans Mono", "Noto Sans Mono", ui-monospace, monospace; font-size: 1.4rem;
          border: 1px solid #8886; padding: .4rem .6rem; width: 48ch; white-space: pre; }
  .meta { font-size: .85rem; color: #666; margin: .25rem 0; }
</style>
</head>
<body>
<h1>yamin-test — reference rendering</h1>
<p>Each box is the case's logical input rendered by the browser with the paragraph direction set. Compare it with what your terminal draws for <code>yamin-test show</code>. Lam-alef width (D1) and cell alignment are not represented here.</p>
"#,
    );
    for cat in Category::ALL {
        s.push_str(&format!("<h2>{}</h2>\n", escape(cat.title())));
        for c in all().iter().filter(|c| c.category == cat) {
            let dir = match c.dir {
                Dir::Ltr => "ltr",
                Dir::Rtl => "rtl",
            };
            s.push_str(&format!(
                "<section id=\"{id}\">\n<h3>{id} · {d}</h3>\n<div class=\"line\" dir=\"{dir}\">{input}</div>\n<p class=\"meta\" dir=\"rtl\">{desc}</p>\n<p class=\"meta\">rules: {rules}</p>\n</section>\n",
                id = c.id,
                d = c.dir.as_str(),
                input = escape(c.input),
                desc = escape(c.description),
                rules = escape(&c.rules.join(" ")),
            ));
        }
    }
    s.push_str("</body>\n</html>\n");
    s
}
