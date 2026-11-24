//! yamin-cat: lay out each input line with yamin and print the visual result.
//!
//! Each line is one paragraph (the spec's hard-newline rule). The output is
//! already reordered and shaped, so it reads correctly in a terminal with no
//! BiDi support at all.

use std::ops::Range;

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use yamin::{cells_from_str, layout_rows, Cell, Direction, Glyph, Props};

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
    /// Keep SGR colors/attributes: each character keeps its style after
    /// reordering. Off: all escape sequences are removed.
    pub color: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            dir: DirMode::Auto,
            width: 80,
            align: true,
            protect: false,
            compact_lamalef: false,
            color: true,
        }
    }
}

const RESET: &str = "\x1b[0m";

/// Drop what a terminal would not store in cells: escape sequences (CSI, OSC
/// and two-byte ESC forms) and C0 controls. Tabs become spaces up to the next
/// 8-column stop.
pub fn clean(line: &str) -> String {
    parse(line).0
}

/// [`clean`], plus the SGR style active at each output character: the SGR
/// sequences seen since the last reset, concatenated ("" when unstyled).
fn parse(line: &str) -> (String, Vec<String>) {
    let mut out = String::with_capacity(line.len());
    let mut styles = Vec::new();
    let mut style = String::new();
    let mut col = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\x1b' => match chars.next() {
                // CSI: parameters/intermediates, then a final byte 0x40..=0x7E.
                // Only SGR (final 'm') is kept, as style state.
                Some('[') => {
                    let mut params = String::new();
                    for c in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c) {
                            if c == 'm' {
                                apply_sgr(&mut style, &params);
                            }
                            break;
                        }
                        params.push(c);
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
                styles.extend(std::iter::repeat_n(style.clone(), spaces));
                col += spaces;
            }
            c if c.is_control() => {}
            c => {
                out.push(c);
                styles.push(style.clone());
                // Marks take no column; wide characters (CJK, emoji) take two.
                col += c.width().unwrap_or(0);
            }
        }
    }
    (out, styles)
}

/// Update the style with one SGR sequence. A reset (`m`, `0m`) clears it; a
/// sequence starting with a reset replaces it; anything else adds to it.
fn apply_sgr(style: &mut String, params: &str) {
    let seq = format!("\x1b[{params}m");
    if params.is_empty() || params == "0" {
        style.clear();
    } else if params.starts_with("0;") {
        *style = seq;
    } else {
        style.push_str(&seq);
    }
}

/// Style of each cell: the style of its first character.
fn cell_styles(cells: &[Cell], char_styles: &[String]) -> Vec<String> {
    let mut i = 0;
    cells
        .iter()
        .map(|cell| {
            let s = char_styles.get(i).cloned().unwrap_or_default();
            if let Cell::Text(t) = cell {
                i += t.chars().count();
            }
            s
        })
        .collect()
}

/// Columns a cell fills on screen: 2 for wide characters, 0 for marks.
fn columns(cell: &Cell) -> usize {
    match cell {
        Cell::Text(t) => t.width(),
        Cell::Erased => 1,
    }
}

/// Greedy wrap by columns: a row holds as many cells as fit in `width`
/// columns; a wide character is never split, and one that is wider than
/// `width` still gets a row of its own.
fn wrap(cells: &[Cell], width: usize) -> Vec<Range<usize>> {
    let mut rows = Vec::new();
    let (mut start, mut used) = (0, 0);
    for (i, cell) in cells.iter().enumerate() {
        let w = columns(cell);
        if i > start && used + w > width {
            rows.push(start..i);
            (start, used) = (i, 0);
        }
        used += w;
    }
    rows.push(start..cells.len());
    rows
}

/// Lay out one line and return its visual rows, top to bottom.
pub fn render(line: &str, opts: &Opts) -> Vec<String> {
    let (text, char_styles) = parse(line);
    let cells = cells_from_str(&text);
    if cells.is_empty() {
        return vec![String::new()];
    }
    let styles = if opts.color {
        cell_styles(&cells, &char_styles)
    } else {
        vec![String::new(); cells.len()]
    };
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
    let l = layout_rows(&cells, &wrap(&cells, width), props);
    (0..l.rows.len())
        .map(|r| {
            // Visual cells to print: compact mode drops lam-alef tails, so the
            // ligature takes one cell instead of leaving a gap beside it.
            let mut used = 0;
            let mut text = String::new();
            // Emit a style change only where the style differs from the
            // previous visual cell; reset before switching and at row end.
            let mut current = "";
            for vc in &l.rows[r] {
                if matches!(vc.glyph, Glyph::LigatureTail) && opts.compact_lamalef {
                    continue;
                }
                let style = styles[vc.logical].as_str();
                if style != current {
                    if !current.is_empty() {
                        text.push_str(RESET);
                    }
                    text.push_str(style);
                    current = style;
                }
                match &vc.glyph {
                    Glyph::Text(t) => {
                        text.push_str(t);
                        used += t.width();
                    }
                    Glyph::LigatureTail | Glyph::Erased => {
                        text.push(' ');
                        used += 1;
                    }
                }
            }
            if !current.is_empty() {
                text.push_str(RESET);
            }
            let mut row = String::new();
            if opts.align && l.direction == Direction::Rtl {
                row.push_str(&" ".repeat(width.saturating_sub(used)));
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
