use yamin::{cells_from_str, layout, Cell, Direction, Mode, Props};

fn explicit(direction: Direction) -> Props {
    Props {
        mode: Mode::Explicit,
        direction,
        autodetect: false,
    }
}

fn auto(fallback: Direction) -> Props {
    Props {
        mode: Mode::Implicit,
        direction: fallback,
        autodetect: true,
    }
}

// ── explicit mode: the app already did BiDi + shaping ─────────────────

#[test]
fn explicit_ltr_is_identity_without_shaping() {
    let l = layout(&cells_from_str("مرحبا"), 5, explicit(Direction::Ltr));
    assert_eq!(l.visual_to_logical(0), vec![0, 1, 2, 3, 4]);
    assert_eq!(l.row_text(0), "مرحبا", "no shaping in explicit mode");
}

#[test]
fn explicit_rtl_reverses_the_whole_row() {
    let l = layout(&cells_from_str("abc"), 3, explicit(Direction::Rtl));
    assert_eq!(l.visual_to_logical(0), vec![2, 1, 0]);
    assert!(l.rows[0].iter().all(|v| v.rtl), "cursor on the right side");
}

#[test]
fn explicit_rtl_reverses_erased_cells_too() {
    let mut c = cells_from_str("ab");
    c.push(Cell::Erased);
    let l = layout(&c, 3, explicit(Direction::Rtl));
    assert_eq!(l.visual_to_logical(0), vec![2, 1, 0]);
}

#[test]
fn explicit_rtl_mirrors_and_ltr_does_not() {
    let rtl = layout(&cells_from_str("(a)"), 3, explicit(Direction::Rtl));
    assert_eq!(rtl.row_text(0), "(a)");
    let ltr = layout(&cells_from_str("(a)"), 3, explicit(Direction::Ltr));
    assert_eq!(ltr.row_text(0), "(a)");
    assert_eq!(ltr.visual_to_logical(0), vec![0, 1, 2]);
}

#[test]
fn explicit_ignores_autodetect() {
    let p = Props {
        mode: Mode::Explicit,
        direction: Direction::Ltr,
        autodetect: true,
    };
    assert_eq!(
        layout(&cells_from_str("مرحبا"), 5, p).direction,
        Direction::Ltr
    );
}

#[test]
fn explicit_rows_are_independent() {
    let l = layout(&cells_from_str("abcd"), 2, explicit(Direction::Rtl));
    assert_eq!(l.visual_to_logical(0), vec![1, 0]);
    assert_eq!(l.visual_to_logical(1), vec![3, 2]);
}

// ── autodetection (implicit only; P2/P3 on the whole paragraph) ──────

#[test]
fn autodetect_first_strong_arabic_gives_rtl() {
    let l = layout(&cells_from_str("مرحبا world"), 11, auto(Direction::Ltr));
    assert_eq!(l.direction, Direction::Rtl);
    assert_eq!(
        l.visual_to_logical(0),
        vec![6, 7, 8, 9, 10, 5, 4, 3, 2, 1, 0]
    );
}

#[test]
fn autodetect_first_strong_latin_gives_ltr() {
    let l = layout(&cells_from_str("hello مرحبا"), 11, auto(Direction::Rtl));
    assert_eq!(l.direction, Direction::Ltr);
}

#[test]
fn autodetect_without_strong_chars_uses_the_fallback() {
    assert_eq!(
        layout(&cells_from_str("123 !"), 5, auto(Direction::Rtl)).direction,
        Direction::Rtl
    );
    assert_eq!(
        layout(&cells_from_str("123 !"), 5, auto(Direction::Ltr)).direction,
        Direction::Ltr
    );
}

#[test]
fn autodetect_looks_past_the_first_row() {
    let l = layout(&cells_from_str("12بت"), 2, auto(Direction::Ltr));
    assert_eq!(l.direction, Direction::Rtl);
}

#[test]
fn autodetect_off_by_default_d5() {
    assert_eq!(
        layout(&cells_from_str("مرحبا"), 5, Props::default()).direction,
        Direction::Ltr
    );
}
