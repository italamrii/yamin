use yamin_test::cases::find;
use yamin_test::display::{cell_width, expected_visual};
use yamin_test::terminal::{codepoints, expected_line, render_case, Opts, LINE_WIDTH};

const LRO: char = '\u{202D}';
const PDF: char = '\u{202C}';

#[test]
fn render_shows_logical_input_raw_for_the_terminal_to_handle() {
    let c = find("mixed-01").unwrap();
    let out = render_case(c, Opts::default());
    assert!(out.contains(c.input));
    assert!(out.contains("mixed-01"));
}

#[test]
fn expected_line_uses_explicit_mode_and_lro_by_default() {
    let c = find("mixed-01").unwrap();
    let line = expected_line(c, Opts::default());
    assert!(line.starts_with("\x1b[8l\x1b[1 k"), "{line:?}");
    assert!(line.contains(&format!("{LRO}{}{PDF}", expected_visual(c))));
    // back to implicit + terminal default direction
    assert!(line.ends_with("\x1b[8h\x1b[0 k"));
}

#[test]
fn no_escapes_and_no_controls_give_plain_text() {
    let c = find("mixed-01").unwrap();
    let opts = Opts {
        escapes: false,
        controls: false,
        codepoints: false,
    };
    let line = expected_line(c, opts);
    assert!(!line.contains('\x1b'));
    assert!(!line.contains(LRO));
    assert_eq!(line.trim_start(), expected_visual(c));
}

#[test]
fn rtl_expected_line_is_right_aligned() {
    let c = find("arabic-01").unwrap();
    let opts = Opts {
        escapes: false,
        controls: false,
        codepoints: false,
    };
    let line = expected_line(c, opts);
    assert_eq!(cell_width(&line), LINE_WIDTH);
}

#[test]
fn ltr_expected_line_is_not_padded() {
    let c = find("mixed-02").unwrap();
    let opts = Opts {
        escapes: false,
        controls: false,
        codepoints: false,
    };
    assert_eq!(expected_line(c, opts), expected_visual(c));
}

#[test]
fn rtl_input_line_sets_paragraph_direction() {
    let c = find("arabic-01").unwrap();
    let out = render_case(c, Opts::default());
    assert!(out.contains(&format!("\x1b[8h\x1b[2 k{}", c.input)));
}

#[test]
fn codepoints_lists_hex() {
    assert_eq!(codepoints("a\u{FEFB}"), "U+0061 U+FEFB");
}
