//! layout_rows: rows of different lengths (e.g. a terminal row holding wide
//! characters has fewer cells than columns).

use yamin::{cells_from_str, layout, layout_rows, Direction, Glyph, Mode, Props};

fn rtl() -> Props {
    Props {
        direction: Direction::Rtl,
        ..Props::default()
    }
}

#[test]
fn uniform_ranges_match_layout() {
    let c = cells_from_str("مرحبا world! سلام");
    let uniform = layout(&c, 6, rtl());
    let ranges: Vec<_> = (0..c.len())
        .step_by(6)
        .map(|s| s..(s + 6).min(c.len()))
        .collect();
    assert_eq!(layout_rows(&c, &ranges, rtl()), uniform);
}

#[test]
fn rows_may_have_different_lengths() {
    let c = cells_from_str("ab cdef");
    let l = layout_rows(&c, &[0..2, 2..7], Props::default());
    assert_eq!(l.visual_to_logical(0), vec![0, 1]);
    assert_eq!(l.visual_to_logical(1), vec![2, 3, 4, 5, 6]);
}

#[test]
fn levels_still_span_the_whole_paragraph() {
    // Same W2 + L1 case as reorder.rs, with uneven rows: "ب  " | "1 2".
    let c = cells_from_str("ب  1 2");
    let l = layout_rows(&c, &[0..3, 3..6], Props::default());
    assert_eq!(l.visual_to_logical(1), vec![5, 4, 3]);
}

#[test]
fn lam_alef_never_ligates_across_a_custom_row_boundary() {
    // سل | ام : the boundary falls between lam and alef.
    let c = cells_from_str("سلام");
    let l = layout_rows(&c, &[0..2, 2..4], rtl());
    assert!(l
        .rows
        .iter()
        .flatten()
        .all(|v| v.glyph != Glyph::LigatureTail));
    // and in one row it does ligate
    let one = layout_rows(&c, std::slice::from_ref(&(0..4)), rtl());
    assert!(one.rows[0].iter().any(|v| v.glyph == Glyph::LigatureTail));
}

#[test]
fn explicit_mode_uses_the_given_rows() {
    let p = Props {
        mode: Mode::Explicit,
        direction: Direction::Rtl,
        autodetect: false,
    };
    let l = layout_rows(&cells_from_str("abcde"), &[0..2, 2..5], p);
    assert_eq!(l.visual_to_logical(0), vec![1, 0]);
    assert_eq!(l.visual_to_logical(1), vec![4, 3, 2]);
}
