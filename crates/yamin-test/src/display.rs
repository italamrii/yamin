//! Builds the expected visual line from a case's runs.

use crate::cases::{Case, Run};
use crate::shaping::{is_mark, shape};

/// Bidi_Mirrored counterparts used by the catalog (UAX#9 L4).
pub fn mirror(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        other => other,
    }
}

/// Render one run in visual order. RTL runs are shaped in logical order, then
/// reversed by cluster (marks stay after their base), then mirrored.
pub fn render_run(run: &Run) -> String {
    match run {
        Run::Ltr(t) => (*t).to_string(),
        Run::Rtl(t) => shape(t)
            .into_iter()
            .rev()
            .map(|cluster| cluster.chars().map(mirror).collect::<String>())
            .collect(),
    }
}

/// The whole expected line, visual left-to-right, without padding.
pub fn expected_visual(case: &Case) -> String {
    case.runs.iter().map(render_run).collect()
}

/// Terminal cells: one per non-mark character (all catalog text is narrow).
pub fn cell_width(s: &str) -> usize {
    s.chars().filter(|&c| !is_mark(c)).count()
}
