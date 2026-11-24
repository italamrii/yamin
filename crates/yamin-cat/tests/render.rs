use yamin_cat::{clean, render, DirMode, Opts};
use yamin_test::cases::{all, Dir};
use yamin_test::display::{cell_width, expected_visual};

fn opts(dir: DirMode, width: usize) -> Opts {
    Opts {
        dir,
        width,
        align: true,
        protect: false,
        compact_lamalef: false,
    }
}

// ── compact lam-alef: one cell instead of two (D1 stays the default) ──

fn compact(width: usize, align: bool) -> Opts {
    Opts {
        compact_lamalef: true,
        align,
        ..opts(DirMode::Auto, width)
    }
}

#[test]
fn default_keeps_the_two_cell_lam_alef_d1() {
    let o = Opts {
        align: false,
        ..opts(DirMode::Auto, 40)
    };
    assert_eq!(render("سلام", &o), vec!["\u{FEE1} \u{FEFC}\u{FEB3}"]);
}

#[test]
fn compact_lam_alef_drops_the_filler_cell() {
    assert_eq!(
        render("سلام", &compact(40, false)),
        vec!["\u{FEE1}\u{FEFC}\u{FEB3}"]
    );
    assert_eq!(render("لا", &compact(40, false)), vec!["\u{FEFB}"]);
}

#[test]
fn compact_lam_alef_still_right_aligns_to_the_full_width() {
    let rows = render("السلام عليكم", &compact(20, true));
    assert_eq!(rows.len(), 1);
    assert_eq!(cell_width(&rows[0]), 20);
    assert!(!rows[0].trim_start().contains("\u{FEFC} "), "{:?}", rows[0]);
}

#[test]
fn compact_does_not_touch_text_without_lam_alef() {
    for c in all().iter().filter(|c| !c.rules.contains(&"D1")) {
        let dir = match c.dir {
            Dir::Ltr => DirMode::Ltr,
            Dir::Rtl => DirMode::Rtl,
        };
        let o = Opts {
            dir,
            ..compact(200, false)
        };
        assert_eq!(
            render(c.input, &o),
            vec![expected_visual(c)],
            "case {}",
            c.id
        );
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
