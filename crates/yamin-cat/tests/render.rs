use yamin_cat::{clean, render, DirMode, Opts};
use yamin_test::cases::{all, Dir};
use yamin_test::display::{cell_width, expected_visual};

fn opts(dir: DirMode, width: usize) -> Opts {
    Opts {
        dir,
        width,
        align: true,
        protect: false,
    }
}

// ── clean: what the terminal would not store in cells ──────────────────

#[test]
fn clean_strips_sgr_and_osc_sequences() {
    assert_eq!(clean("\x1b[1;31mred\x1b[0m"), "red");
    assert_eq!(clean("\x1b]8;;https://x\x07link\x1b]8;;\x1b\\"), "link");
}

#[test]
fn clean_expands_tabs_to_8_column_stops() {
    assert_eq!(clean("a\tb"), format!("a{}b", " ".repeat(7)));
    assert_eq!(clean("\tb"), format!("{}b", " ".repeat(8)));
}

#[test]
fn clean_drops_carriage_returns_and_controls() {
    assert_eq!(clean("ab\r"), "ab");
    assert_eq!(clean("a\x07b"), "ab");
}

// ── render ─────────────────────────────────────────────────────────────

#[test]
fn latin_text_is_unchanged() {
    assert_eq!(
        render("hello world", &opts(DirMode::Auto, 80)),
        vec!["hello world"]
    );
}

#[test]
fn empty_line_stays_empty() {
    assert_eq!(render("", &opts(DirMode::Auto, 80)), vec![""]);
}

#[test]
fn auto_detects_rtl_and_right_aligns_to_the_width() {
    let rows = render("مرحبا world!", &opts(DirMode::Auto, 40));
    assert_eq!(rows.len(), 1);
    assert_eq!(cell_width(&rows[0]), 40);
    assert!(rows[0].ends_with("!world \u{FE8E}\u{FE92}\u{FEA3}\u{FEAE}\u{FEE3}"));
}

#[test]
fn no_align_means_no_padding() {
    let o = Opts {
        align: false,
        ..opts(DirMode::Auto, 40)
    };
    assert_eq!(
        render("مرحبا", &o),
        vec!["\u{FE8E}\u{FE92}\u{FEA3}\u{FEAE}\u{FEE3}"]
    );
}

#[test]
fn forced_ltr_keeps_left_alignment() {
    assert_eq!(
        render("مرحبا", &opts(DirMode::Ltr, 40)),
        vec!["\u{FE8E}\u{FE92}\u{FEA3}\u{FEAE}\u{FEE3}"]
    );
}

#[test]
fn long_rtl_lines_wrap_at_the_width_first_words_on_top() {
    let rows = render("مرحبا بالعالم", &opts(DirMode::Auto, 6));
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().all(|r| cell_width(r) == 6), "{rows:?}");
    // the first logical letter (م, initial form) is the rightmost cell of row 0
    assert!(rows[0].ends_with('\u{FEE3}'));
}

#[test]
fn protect_wraps_each_row_in_lro_pdf() {
    let o = Opts {
        protect: true,
        align: false,
        ..opts(DirMode::Auto, 40)
    };
    let rows = render("ab", &o);
    assert_eq!(rows, vec!["\u{202D}ab\u{202C}"]);
}

#[test]
fn all_40_conformance_cases_render_as_expected() {
    for c in all() {
        let dir = match c.dir {
            Dir::Ltr => DirMode::Ltr,
            Dir::Rtl => DirMode::Rtl,
        };
        let o = Opts {
            align: false,
            ..opts(dir, 200)
        };
        assert_eq!(
            render(c.input, &o),
            vec![expected_visual(c)],
            "case {}",
            c.id
        );
    }
}
