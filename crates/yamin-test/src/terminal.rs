//! Terminal output for one case: the raw input (rendered by the terminal under
//! test) and the expected visual line (pre-ordered, explicit mode).

use crate::cases::{Case, Dir};
use crate::display::{cell_width, expected_visual};

/// Default width RTL expected lines are right-aligned to, when the terminal
/// width is unknown.
pub const LINE_WIDTH: usize = 48;

const LRO: char = '\u{202D}';
const PDF: char = '\u{202C}';

/// BDSM implicit / explicit (ECMA-48 SM/RM 8).
const IMPLICIT: &str = "\x1b[8h";
const EXPLICIT: &str = "\x1b[8l";
/// SCP: terminal default / LTR / RTL.
const SCP_DEFAULT: &str = "\x1b[0 k";
const SCP_LTR: &str = "\x1b[1 k";
const SCP_RTL: &str = "\x1b[2 k";

#[derive(Clone, Copy, Debug)]
pub struct Opts {
    /// Emit BDSM/SCP escape sequences (spec section 3).
    pub escapes: bool,
    /// Wrap the expected line in LRO..PDF.
    pub controls: bool,
    /// Also print the expected line's code points.
    pub codepoints: bool,
    /// Terminal width in cells; RTL expected lines are right-aligned to it.
    /// Padding past the real width would wrap the line.
    pub width: usize,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            escapes: true,
            controls: true,
            codepoints: false,
            width: LINE_WIDTH,
        }
    }
}

/// The expected line in visual order. Printed in explicit LTR mode so a
/// conforming terminal draws it as-is; LRO..PDF protects it in terminals that
/// apply BiDi anyway. RTL paragraphs are right-aligned to `opts.width`.
pub fn expected_line(case: &Case, opts: Opts) -> String {
    let visual = expected_visual(case);
    let pad = match case.dir {
        Dir::Rtl => opts.width.saturating_sub(cell_width(&visual)),
        Dir::Ltr => 0,
    };
    let mut s = String::new();
    if opts.escapes {
        s.push_str(EXPLICIT);
        s.push_str(SCP_LTR);
    }
    s.push_str(&" ".repeat(pad));
    if opts.controls {
        s.push(LRO);
    }
    s.push_str(&visual);
    if opts.controls {
        s.push(PDF);
    }
    if opts.escapes {
        // Applied to the next paragraph (spec: "How these escape sequences apply").
        s.push_str(IMPLICIT);
        s.push_str(SCP_DEFAULT);
    }
    s
}

/// The input line as the terminal under test should render it (implicit mode,
/// paragraph direction set with SCP).
pub fn input_line(case: &Case, opts: Opts) -> String {
    if !opts.escapes {
        return case.input.to_string();
    }
    let scp = match case.dir {
        Dir::Ltr => SCP_LTR,
        Dir::Rtl => SCP_RTL,
    };
    format!("{IMPLICIT}{scp}{}{SCP_DEFAULT}", case.input)
}

pub fn render_case(case: &Case, opts: Opts) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "━━ [{}] {} · {} ━━\n",
        case.id,
        case.category.title(),
        case.dir.as_str()
    ));
    s.push_str(&format!("rules: {}\n", case.rules.join(" ")));
    s.push_str(&format!("{}\n", case.description));
    s.push_str("── actual (your terminal, implicit mode):\n");
    s.push_str(&input_line(case, opts));
    s.push('\n');
    s.push_str("── expected (visual order, explicit mode):\n");
    s.push_str(&expected_line(case, opts));
    s.push('\n');
    if opts.codepoints {
        s.push_str(&format!(
            "codepoints: {}\n",
            codepoints(&expected_visual(case))
        ));
    }
    s
}

pub fn codepoints(s: &str) -> String {
    s.chars()
        .map(|c| format!("U+{:04X}", c as u32))
        .collect::<Vec<_>>()
        .join(" ")
}
