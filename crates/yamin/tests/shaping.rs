use yamin::{cells_from_str, layout, Cell, Direction, Glyph, Props};

fn rtl() -> Props {
    Props { direction: Direction::Rtl, ..Props::default() }
}

fn rows(text: &str, width: usize, props: Props) -> Vec<String> {
    let cells = cells_from_str(text);
    let l = layout(&cells, width, props);
    (0..l.rows.len()).map(|r| l.row_text(r)).collect()
}

fn s(cps: &[u32]) -> String {
    cps.iter().map(|&c| char::from_u32(c).unwrap()).collect()
}

#[test]
fn positional_forms_in_visual_order() {
    // مرحبا: م initial, ر final, ح initial, ب medial, ا final — drawn right to left.
    assert_eq!(rows("مرحبا", 5, rtl()), vec![s(&[0xFE8E, 0xFE92, 0xFEA3, 0xFEAE, 0xFEE3])]);
}

#[test]
fn marks_are_transparent_and_stay_on_their_base() {
    assert_eq!(
        rows("بِسْمِ", 3, rtl()),
        vec![s(&[0xFEE2, 0x0650, 0xFEB4, 0x0652, 0xFE91, 0x0650])]
    );
}

#[test]
fn lam_alef_ligature_spans_two_cells_d1() {
    let cells = cells_from_str("سلام");
    let l = layout(&cells, 4, rtl());
    // visual: م | (alef tail) | lam-alef final | س initial
    assert_eq!(l.visual_to_logical(0), vec![3, 2, 1, 0]);
    assert_eq!(l.rows[0][1].glyph, Glyph::LigatureTail);
    assert_eq!(l.rows[0][2].glyph, Glyph::Text("\u{FEFC}".into()));
    assert_eq!(l.row_text(0), s(&[0xFEE1, 0x20, 0xFEFC, 0xFEB3]));
}

#[test]
fn lam_alef_variants_isolated() {
    assert_eq!(rows("لأ", 2, rtl())[0], s(&[0x20, 0xFEF7]));
    assert_eq!(rows("لإ", 2, rtl())[0], s(&[0x20, 0xFEF9]));
    assert_eq!(rows("لآ", 2, rtl())[0], s(&[0x20, 0xFEF5]));
    assert_eq!(rows("لا", 2, rtl())[0], s(&[0x20, 0xFEFB]));
}

#[test]
fn shaping_runs_on_the_paragraph_across_soft_wrap_d3() {
    // بب wrapped at width 1: initial on row 0, final on row 1.
    assert_eq!(rows("بب", 1, rtl()), vec![s(&[0xFE91]), s(&[0xFE90])]);
}

#[test]
fn lam_alef_split_by_wrap_does_not_ligate() {
    // بل | ا : lam joins both sides (medial), alef is final; no ligature.
    assert_eq!(rows("بلا", 2, rtl()), vec![s(&[0xFEE0, 0xFE91]), s(&[0xFE8E])]);
}

#[test]
fn erased_cells_break_joining() {
    let mut c = cells_from_str("ب");
    c.push(Cell::Erased);
    c.extend(cells_from_str("ب"));
    let l = layout(&c, 3, rtl());
    // Both isolated; the erased cell moves to the row end (left in RTL).
    assert_eq!(l.row_text(0), s(&[0x20, 0xFE8F, 0xFE8F]));
}

#[test]
fn persian_letters_have_forms() {
    // پی: پ initial, ی final
    assert_eq!(rows("پی", 2, rtl())[0], s(&[0xFBFD, 0xFB58]));
}

#[test]
fn tatweel_joins_and_is_kept() {
    // بـ: ب initial, tatweel unchanged
    assert_eq!(rows("بـ", 2, rtl())[0], s(&[0x0640, 0xFE91]));
}

#[test]
fn presentation_forms_pass_through_d6() {
    assert_eq!(rows("\u{FEE3}\u{FEAE}", 2, rtl())[0], s(&[0xFEAE, 0xFEE3]));
}

#[test]
fn brackets_mirror_only_when_resolved_rtl_l4() {
    // RTL paragraph: (ب) — brackets are R, reversed and mirrored.
    assert_eq!(rows("(ب)", 3, rtl())[0], s(&[0x28, 0xFE8F, 0x29]));
    // LTR paragraph: call (ب) — brackets are L, not mirrored.
    assert_eq!(rows("x (ب)", 5, Props::default())[0], s(&[0x78, 0x20, 0x28, 0xFE8F, 0x29]));
    // Latin in LTR is untouched.
    assert_eq!(rows("(a)", 3, Props::default())[0], "(a)");
}
