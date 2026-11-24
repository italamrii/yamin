//! Arabic shaping on the model (logical order), per paragraph (spec D3).
//!
//! Each cell's base character gets its positional presentation form from its
//! neighbouring cells. Marks live inside cells, so they are transparent for
//! free (D4). Lam + alef in the same row becomes a ligature on the lam cell and
//! a tail on the alef cell (D1). Presentation forms already in the input are
//! left alone (D6).

use crate::{Cell, Glyph};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Join {
    /// Isolated, final, initial, medial.
    Dual,
    /// Isolated, final.
    Right,
    /// Tatweel / ZWJ: joins both sides, has no forms of its own.
    Causing,
    None,
}

/// Joining type and the isolated presentation form (the other forms follow it
/// in order: final, initial, medial).
fn info(c: char) -> (Join, Option<u32>) {
    use Join::*;
    let (j, isol) = match c as u32 {
        0x0621 => (None, 0xFE80),
        0x0622 => (Right, 0xFE81),
        0x0623 => (Right, 0xFE83),
        0x0624 => (Right, 0xFE85),
        0x0625 => (Right, 0xFE87),
        0x0626 => (Dual, 0xFE89),
        0x0627 => (Right, 0xFE8D),
        0x0628 => (Dual, 0xFE8F),
        0x0629 => (Right, 0xFE93),
        0x062A => (Dual, 0xFE95),
        0x062B => (Dual, 0xFE99),
        0x062C => (Dual, 0xFE9D),
        0x062D => (Dual, 0xFEA1),
        0x062E => (Dual, 0xFEA5),
        0x062F => (Right, 0xFEA9),
        0x0630 => (Right, 0xFEAB),
        0x0631 => (Right, 0xFEAD),
        0x0632 => (Right, 0xFEAF),
        0x0633 => (Dual, 0xFEB1),
        0x0634 => (Dual, 0xFEB5),
        0x0635 => (Dual, 0xFEB9),
        0x0636 => (Dual, 0xFEBD),
        0x0637 => (Dual, 0xFEC1),
        0x0638 => (Dual, 0xFEC5),
        0x0639 => (Dual, 0xFEC9),
        0x063A => (Dual, 0xFECD),
        0x0640 | 0x200D => return (Causing, Option::None),
        0x0641 => (Dual, 0xFED1),
        0x0642 => (Dual, 0xFED5),
        0x0643 => (Dual, 0xFED9),
        0x0644 => (Dual, 0xFEDD),
        0x0645 => (Dual, 0xFEE1),
        0x0646 => (Dual, 0xFEE5),
        0x0647 => (Dual, 0xFEE9),
        0x0648 => (Right, 0xFEED),
        0x0649 => (Right, 0xFEEF),
        0x064A => (Dual, 0xFEF1),
        // Persian / Urdu letters with Arabic Presentation Forms-A.
        0x067E => (Dual, 0xFB56),  // پ
        0x0686 => (Dual, 0xFB7A),  // چ
        0x0698 => (Right, 0xFB8A), // ژ
        0x06A9 => (Dual, 0xFB8E),  // ک
        0x06AF => (Dual, 0xFB92),  // گ
        0x06CC => (Dual, 0xFBFC),  // ی
        _ => return (None, Option::None),
    };
    (j, Some(isol))
}

fn joins_left(j: Join) -> bool {
    matches!(j, Join::Dual | Join::Causing)
}

fn joins_right(j: Join) -> bool {
    matches!(j, Join::Dual | Join::Right | Join::Causing)
}

/// Isolated lam-alef ligature for an alef variant (final form is +1).
fn lam_alef(alef: char) -> Option<u32> {
    match alef as u32 {
        0x0622 => Some(0xFEF5),
        0x0623 => Some(0xFEF7),
        0x0625 => Some(0xFEF9),
        0x0627 => Some(0xFEFB),
        _ => None,
    }
}

fn base(cell: &Cell) -> Option<char> {
    match cell {
        Cell::Text(t) => t.chars().next(),
        Cell::Erased => None,
    }
}

fn join(cell: &Cell) -> Join {
    base(cell).map_or(Join::None, |c| info(c).0)
}

/// Replace the first char of `text` with `glyph`, keeping the marks.
fn with_base(glyph: char, text: &str) -> String {
    let mut s = String::from(glyph);
    s.extend(text.chars().skip(1));
    s
}

/// Shape a paragraph. `width` is the row width: a ligature never spans rows.
pub(crate) fn shape(cells: &[Cell], width: usize) -> Vec<Glyph> {
    let n = cells.len();
    let mut out: Vec<Glyph> = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        let text = match &cells[i] {
            Cell::Text(t) => t,
            Cell::Erased => {
                out.push(Glyph::Erased);
                i += 1;
                continue;
            }
        };
        let c = text.chars().next().unwrap_or(' ');
        let (j, isol) = info(c);
        // Adjacent cells only: an erased cell breaks the join.
        let joins_prev = joins_right(j) && i > 0 && joins_left(join(&cells[i - 1]));

        if c == '\u{0644}' && i + 1 < n && (i + 1) % width != 0 {
            if let Some((alef_text, lig)) = match &cells[i + 1] {
                Cell::Text(a) => a.chars().next().and_then(lam_alef).map(|l| (a, l)),
                Cell::Erased => None,
            } {
                let form = char::from_u32(if joins_prev { lig + 1 } else { lig }).unwrap();
                let mut s = with_base(form, text);
                s.extend(alef_text.chars().skip(1)); // the alef's marks
                out.push(Glyph::Text(s));
                out.push(Glyph::LigatureTail);
                i += 2;
                continue;
            }
        }

        let joins_next = joins_left(j) && i + 1 < n && joins_right(join(&cells[i + 1]));
        let glyph = match isol {
            None => text.clone(),
            Some(isol) => {
                let offset = match (j, joins_prev, joins_next) {
                    (Join::Dual, true, true) => 3,
                    (Join::Dual, false, true) => 2,
                    (_, true, _) => 1,
                    _ => 0,
                };
                with_base(char::from_u32(isol + offset).unwrap(), text)
            }
        };
        out.push(Glyph::Text(glyph));
        i += 1;
    }
    out
}
