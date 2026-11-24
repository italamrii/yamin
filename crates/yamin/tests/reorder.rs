use yamin::{cells_from_str, layout, Cell, Direction, Props};

fn rtl() -> Props {
    Props {
        direction: Direction::Rtl,
        ..Props::default()
    }
}

fn order(cells: &[Cell], width: usize, props: Props) -> Vec<Vec<usize>> {
    let l = layout(cells, width, props);
    (0..l.rows.len()).map(|r| l.visual_to_logical(r)).collect()
}

#[test]
fn cells_keep_marks_with_their_base() {
    let c = cells_from_str("بِسْم a");
    assert_eq!(c.len(), 5);
    assert_eq!(c[0], Cell::Text("بِ".into()));
    assert_eq!(c[1], Cell::Text("سْ".into()));
}

#[test]
fn ltr_latin_is_identity() {
    assert_eq!(
        order(&cells_from_str("abc"), 3, Props::default()),
        vec![vec![0, 1, 2]]
    );
}

#[test]
fn rtl_paragraph_with_latin_word_and_trailing_bang() {
    // "مرحبا world!" — mixed-05: ! goes to the far left.
    let c = cells_from_str("مرحبا world!");
    assert_eq!(
        order(&c, 12, rtl()),
        vec![vec![11, 6, 7, 8, 9, 10, 5, 4, 3, 2, 1, 0]]
    );
}

#[test]
fn marks_stay_inside_their_cell_after_reversal() {
    let l = layout(&cells_from_str("بِسْ"), 2, rtl());
    assert_eq!(l.visual_to_logical(0), vec![1, 0]);
    assert!(l.rows[0].iter().all(|v| v.rtl));
}

#[test]
fn erased_cells_go_to_the_end_in_paragraph_direction() {
    let mut c = cells_from_str("ab");
    c.extend([Cell::Erased, Cell::Erased]);
    // RTL: the end is the left side; text keeps LTR order inside.
    assert_eq!(order(&c, 4, rtl()), vec![vec![3, 2, 0, 1]]);
    // LTR: the end is the right side.
    assert_eq!(order(&c, 4, Props::default()), vec![vec![0, 1, 2, 3]]);
}

#[test]
fn rtl_text_is_right_aligned_by_trailing_erased_cells() {
    let mut c = cells_from_str("بت");
    c.extend([Cell::Erased, Cell::Erased]);
    assert_eq!(order(&c, 4, rtl()), vec![vec![3, 2, 1, 0]]);
}

#[test]
fn levels_use_the_whole_paragraph_but_reorder_per_row() {
    // LTR paragraph, two rows of 3: "ب  " | "1 2".
    // W2: the digits follow ب (AL) in the *previous row*, so they are AN and the
    // space between them resolves R: row 1 shows "2 1".
    // L1: row 0's trailing spaces return to the paragraph level (stay on the right).
    let c = cells_from_str("ب  1 2");
    assert_eq!(
        order(&c, 3, Props::default()),
        vec![vec![0, 1, 2], vec![5, 4, 3]]
    );
}

#[test]
fn logical_to_visual_is_the_inverse() {
    let l = layout(&cells_from_str("مرحبا world!"), 12, rtl());
    let v2l = l.visual_to_logical(0);
    let l2v = l.logical_to_visual(0);
    for (vis, &log) in v2l.iter().enumerate() {
        assert_eq!(l2v[log], vis);
    }
}

#[test]
fn row_text_without_shaping_hooks_is_plain_reordered_text() {
    let l = layout(&cells_from_str("ab"), 2, Props::default());
    assert_eq!(l.row_text(0), "ab");
    assert_eq!(l.direction, Direction::Ltr);
}
