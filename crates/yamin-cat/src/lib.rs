//! yamin-cat: lay out each input line with yamin and print the visual result.
//!
//! Each line is one paragraph (the spec's hard-newline rule). The output is
//! already reordered and shaped, so it reads correctly in a terminal with no
//! BiDi support at all.

use yamin::{cells_from_str, layout, Direction, Glyph, Props};

const LRO: char = '\u{202D}';
const PDF: char = '\u{202C}';
const TAB_STOP: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirMode {
    Ltr,
    Rtl,
    /// First strong character decides (P2/P3); LTR if none.
    Auto,
}

#[derive(Clone, Copy, Debug)]
pub struct Opts {
    pub dir: DirMode,
    /// Terminal width in cells: rows wrap here, RTL rows right-align to it.
    pub width: usize,
    pub align: bool,
    /// Wrap each row in LRO..PDF, for terminals that apply BiDi themselves.
    pub protect: bool,
    /// Draw lam-alef in one cell instead of two (D1). For terminals without
    /// BiDi, where the second cell otherwise shows as a visible gap.
    pub compact_lamalef: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            dir: DirMode::Auto,
            width: 80,
            align: true,
            protect: false,
            compact_lamalef: false,
        }
    }
}

/// Drop what a terminal would not store in cells: escape sequences (CSI, OSC
/// and two-byte ESC forms) and C0 controls. Tabs become spaces up to the next
/// 8-column stop.
pub fn clean(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut col = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\x1b' => match chars.next() {
                // CSI: parameters/intermediates, then a final byte 0x40..=0x7E.
                Some('[') => {
                    for c in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c) {
                            break;
                        }
                    }
                }
                // OSC: ends with BEL or ST (ESC \).
                Some(']') => {
                    while let Some(c) = chars.next() {
                        if c == '\x07' {
                            break;
                        }
                        if c == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {}
            },
            '\t' => {
                let spaces = TAB_STOP - col % TAB_STOP;
                out.extend(std::iter::repeat_n(' ', spaces));
                col += spaces;
            }
            c if c.is_control() => {}
            c => {
                out.push(c);
                if !is_mark(c) {
                    col += 1;
                }
            }
        }
    }
    out
}

/// Combining marks take no column (they live in their base's cell).
fn is_mark(c: char) -> bool {
    // Same test yamin uses to build cells: a mark joins the previous cell.
    cells_from_str(&format!("a{c}")).len() == 1
}

/// Lay out one line and return its visual rows, top to bottom.
pub fn render(line: &str, opts: &Opts) -> Vec<String> {
    let cells = cells_from_str(&clean(line));
    if cells.is_empty() {
        return vec![String::new()];
    }
    let width = opts.width.max(1);
    let props = match opts.dir {
        DirMode::Ltr => Props {
            direction: Direction::Ltr,
            ..Props::default()
        },
        DirMode::Rtl => Props {
            direction: Direction::Rtl,
            ..Props::default()
        },
        DirMode::Auto => Props {
            autodetect: true,
            ..Props::default()
        },
    };
    let l = layout(&cells, width, props);
    (0..l.rows.len())
        .map(|r| {
            // Visual cells to print: compact mode drops lam-alef tails, so the
            // ligature takes one cell instead of leaving a gap beside it.
            let mut cells = 0;
            let mut text = String::new();
            for vc in &l.rows[r] {
                match &vc.glyph {
                    Glyph::Text(t) => text.push_str(t),
                    Glyph::LigatureTail if opts.compact_lamalef => continue,
                    Glyph::LigatureTail | Glyph::Erased => text.push(' '),
                }
                cells += 1;
            }
            let mut row = String::new();
            if opts.align && l.direction == Direction::Rtl {
                row.push_str(&" ".repeat(width.saturating_sub(cells)));
            }
            if opts.protect {
                row.push(LRO);
            }
            row.push_str(&text);
            if opts.protect {
                row.push(PDF);
            }
            row
        })
        .collect()
}
