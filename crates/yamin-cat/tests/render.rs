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
        color: true,
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

// ── wide characters (CJK, emoji) take two columns ─────────────────────

#[test]
fn wide_chars_wrap_by_columns_not_cells() {
    let o = Opts {
        align: false,
        ..opts(DirMode::Ltr, 5)
    };
    assert_eq!(render("你好世界", &o), vec!["你好", "世界"]);
}

#[test]
fn a_wide_char_is_never_split() {
    let o = Opts {
        align: false,
        ..opts(DirMode::Ltr, 2)
    };
    assert_eq!(render("a你", &o), vec!["a", "你"]);
}

#[test]
fn a_wide_char_wider_than_the_width_gets_its_own_row() {
    let o = Opts {
        align: false,
        ..opts(DirMode::Ltr, 1)
    };
    assert_eq!(render("你a", &o), vec!["你", "a"]);
}

#[test]
fn rtl_padding_counts_emoji_as_two_columns() {
    // "😀 مرحبا" fills 2 + 1 + 5 = 8 columns: 12 columns of padding at width 20.
    let rows = render("مرحبا 😀", &opts(DirMode::Auto, 20));
    assert_eq!(rows.len(), 1);
    let pad = rows[0].len() - rows[0].trim_start().len();
    assert_eq!(pad, 12, "{:?}", rows[0]);
    assert!(rows[0].trim_start().starts_with('😀'));
}

#[test]
fn tabs_count_wide_chars_as_two_columns() {
    assert_eq!(clean("你\tb"), format!("你{}b", " ".repeat(6)));
}

// ── colors (SGR) follow their characters through reordering ───────────

fn plain(dir: DirMode) -> Opts {
    Opts {
        align: false,
        ..opts(dir, 80)
    }
}

#[test]
fn colored_latin_keeps_its_color() {
    assert_eq!(
        render("\x1b[31mred\x1b[0m", &plain(DirMode::Auto)),
        vec!["\x1b[31mred\x1b[0m"]
    );
}

#[test]
fn color_follows_the_arabic_word_after_reordering() {
    // Logical: [red]مرحبا[reset] world  →  visual: world ⟨red⟩ابحرم⟨reset⟩
    let rows = render("\x1b[31mمرحبا\x1b[0m world", &plain(DirMode::Rtl));
    assert_eq!(
        rows,
        vec!["world \x1b[31m\u{FE8E}\u{FE92}\u{FEA3}\u{FEAE}\u{FEE3}\x1b[0m"]
    );
}

#[test]
fn stacked_attributes_are_kept_together() {
    assert_eq!(
        render("\x1b[1m\x1b[31mab\x1b[0m", &plain(DirMode::Ltr)),
        vec!["\x1b[1m\x1b[31mab\x1b[0m"]
    );
}

#[test]
fn a_color_change_inside_a_word_does_not_break_joining_d2() {
    let rows = render("\x1b[31mم\x1b[32mرحبا\x1b[0m", &plain(DirMode::Rtl));
    // م keeps its initial form although its neighbour has another color.
    assert!(rows[0].contains('\u{FEE3}'), "{:?}", rows[0]);
    assert!(rows[0].contains("\x1b[31m\u{FEE3}"), "{:?}", rows[0]);
}

#[test]
fn no_color_strips_escapes() {
    let o = Opts {
        color: false,
        ..plain(DirMode::Auto)
    };
    assert_eq!(render("\x1b[31mred\x1b[0m", &o), vec!["red"]);
}

#[test]
fn uncolored_text_gets_no_escapes() {
    assert_eq!(render("plain", &plain(DirMode::Auto)), vec!["plain"]);
}
