use yamin_test::cases::{all, Dir, Run};
use yamin_test::display::{cell_width, expected_visual, mirror, render_run};

#[test]
fn ltr_run_is_unchanged() {
    assert_eq!(render_run(&Run::Ltr("git commit")), "git commit");
}

#[test]
fn rtl_run_is_shaped_reversed_and_mirrored() {
    let expected: String = ['(', '\u{FE8E}', '\u{FE92}', '\u{FEA3}', '\u{FEAE}', '\u{FEE3}', ')']
        .iter()
        .collect();
    assert_eq!(render_run(&Run::Rtl("(مرحبا)")), expected);
}

#[test]
fn rtl_reversal_keeps_marks_after_their_base() {
    assert_eq!(
        render_run(&Run::Rtl("بِسْمِ")),
        "\u{FEE2}\u{0650}\u{FEB4}\u{0652}\u{FE91}\u{0650}"
    );
}

#[test]
fn mirror_pairs() {
    for (a, b) in [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>'), ('«', '»')] {
        assert_eq!(mirror(a), b);
        assert_eq!(mirror(b), a);
    }
    assert_eq!(mirror('x'), 'x');
}

#[test]
fn cell_width_ignores_marks() {
    assert_eq!(cell_width("a\u{0650}b"), 2);
}

#[test]
fn expected_visual_width_matches_input_width() {
    // Shaping keeps one cell per base char (lam-alef uses a filler, D1).
    for c in all() {
        assert_eq!(
            cell_width(&expected_visual(c)),
            cell_width(c.input),
            "case {}",
            c.id
        );
    }
}

#[test]
fn classic_exclamation_case_puts_bang_on_the_left() {
    let c = all().iter().find(|c| c.id == "mixed-05").unwrap();
    assert_eq!(c.dir, Dir::Rtl);
    assert!(expected_visual(c).starts_with("!world"));
}
