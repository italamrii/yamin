//! Test-oracle Arabic shaping (logical order -> presentation forms).
//!
//! Independent from the future `yamin` library on purpose: it is the oracle
//! that `yamin` will be checked against. Covers the basic Arabic letters
//! (U+0621..U+064A), harakat as transparent marks, and lam-alef (D1).

/// D1: a lam-alef ligature keeps two cells; the second cell is this filler.
pub const LAM_ALEF_FILLER: char = ' ';

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Join {
    /// Dual-joining: isolated, final, initial, medial.
    Dual,
    /// Right-joining: isolated, final.
    Right,
    /// Join-causing (tatweel): no forms of its own.
    Causing,
    /// Non-joining.
    None,
    /// Transparent (combining marks).
    Transparent,
}

/// (joining type, isolated presentation form). Forms are consecutive:
/// isolated, final, initial, medial.
fn info(c: char) -> (Join, Option<u32>) {
    let (j, isol) = match c as u32 {
        0x0621 => (Join::None, 0xFE80),
        0x0622 => (Join::Right, 0xFE81),
        0x0623 => (Join::Right, 0xFE83),
        0x0624 => (Join::Right, 0xFE85),
        0x0625 => (Join::Right, 0xFE87),
        0x0626 => (Join::Dual, 0xFE89),
        0x0627 => (Join::Right, 0xFE8D),
        0x0628 => (Join::Dual, 0xFE8F),
        0x0629 => (Join::Right, 0xFE93),
        0x062A => (Join::Dual, 0xFE95),
        0x062B => (Join::Dual, 0xFE99),
        0x062C => (Join::Dual, 0xFE9D),
        0x062D => (Join::Dual, 0xFEA1),
        0x062E => (Join::Dual, 0xFEA5),
        0x062F => (Join::Right, 0xFEA9),
        0x0630 => (Join::Right, 0xFEAB),
        0x0631 => (Join::Right, 0xFEAD),
        0x0632 => (Join::Right, 0xFEAF),
        0x0633 => (Join::Dual, 0xFEB1),
        0x0634 => (Join::Dual, 0xFEB5),
        0x0635 => (Join::Dual, 0xFEB9),
        0x0636 => (Join::Dual, 0xFEBD),
        0x0637 => (Join::Dual, 0xFEC1),
        0x0638 => (Join::Dual, 0xFEC5),
        0x0639 => (Join::Dual, 0xFEC9),
        0x063A => (Join::Dual, 0xFECD),
        0x0640 => return (Join::Causing, None),
        0x0641 => (Join::Dual, 0xFED1),
        0x0642 => (Join::Dual, 0xFED5),
        0x0643 => (Join::Dual, 0xFED9),
        0x0644 => (Join::Dual, 0xFEDD),
        0x0645 => (Join::Dual, 0xFEE1),
        0x0646 => (Join::Dual, 0xFEE5),
        0x0647 => (Join::Dual, 0xFEE9),
        0x0648 => (Join::Right, 0xFEED),
        0x0649 => (Join::Right, 0xFEEF),
        0x064A => (Join::Dual, 0xFEF1),
        _ if is_mark(c) => return (Join::Transparent, None),
        _ => return (Join::None, None),
    };
    (j, Some(isol))
}

/// Harakat and other Arabic combining marks used in the catalog.
pub fn is_mark(c: char) -> bool {
    matches!(c as u32, 0x064B..=0x065F | 0x0670)
}

fn joins_left(j: Join) -> bool {
    matches!(j, Join::Dual | Join::Causing)
}

fn joins_right(j: Join) -> bool {
    matches!(j, Join::Dual | Join::Right | Join::Causing)
}

/// Isolated lam-alef ligature for lam + `alef`, if `alef` is an alef variant.
fn lam_alef(alef: char) -> Option<u32> {
    match alef as u32 {
        0x0622 => Some(0xFEF5),
        0x0623 => Some(0xFEF7),
        0x0625 => Some(0xFEF9),
        0x0627 => Some(0xFEFB),
        _ => None,
    }
}

/// Shape logical text into clusters (one per cell): a base glyph followed by
/// its marks. Arabic letters become presentation forms (U+FExx).
pub fn shape(logical: &str) -> Vec<String> {
    let chars: Vec<char> = logical.chars().collect();
    let join = |i: usize| info(chars[i]).0;
    let prev_base = |i: usize| (0..i).rev().find(|&k| join(k) != Join::Transparent);
    let next_base = |i: usize| (i + 1..chars.len()).find(|&k| join(k) != Join::Transparent);

    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let (j, isol) = info(c);

        if j == Join::Transparent {
            match out.last_mut() {
                Some(last) => last.push(c),
                None => out.push(c.to_string()),
            }
            i += 1;
            continue;
        }

        let joins_prev = joins_right(j) && prev_base(i).is_some_and(|p| joins_left(join(p)));

        // Lam directly followed by an alef variant -> ligature + filler (D1).
        if c == '\u{0644}' {
            if let Some(lig) = chars.get(i + 1).and_then(|&a| lam_alef(a)) {
                let form = if joins_prev { lig + 1 } else { lig };
                out.push(char::from_u32(form).unwrap().to_string());
                out.push(LAM_ALEF_FILLER.to_string());
                i += 2;
                continue;
            }
        }

        let joins_next = joins_left(j) && next_base(i).is_some_and(|n| joins_right(join(n)));
        let glyph = match isol {
            None => c,
            Some(base) => {
                let offset = match (j, joins_prev, joins_next) {
                    (Join::Dual, true, true) => 3,
                    (Join::Dual, false, true) => 2,
                    (_, true, _) => 1,
                    _ => 0,
                };
                char::from_u32(base + offset).unwrap()
            }
        };
        out.push(glyph.to_string());
        i += 1;
    }
    out
}
