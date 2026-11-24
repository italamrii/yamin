//! Cross-checks every case's expected visual *order* against `unicode-bidi`,
//! an independent UAX#9 implementation. Shaping and mirroring are excluded:
//! both sides are compared as plain code points, RTL runs reversed per char.

use unicode_bidi::{BidiInfo, Level};
use yamin_test::cases::{all, Dir, Run};

fn ours(runs: &[Run]) -> String {
    runs.iter()
        .map(|r| match r {
            Run::Ltr(t) => t.to_string(),
            Run::Rtl(t) => t.chars().rev().collect(),
        })
        .collect()
}

fn reference(input: &str, dir: Dir) -> String {
    let level = match dir {
        Dir::Ltr => Level::ltr(),
        Dir::Rtl => Level::rtl(),
    };
    let info = BidiInfo::new(input, Some(level)); // HL1: paragraph level set explicitly
    let para = &info.paragraphs[0];
    info.reorder_line(para, para.range.clone()).into_owned()
}

#[test]
fn expected_order_matches_unicode_bidi() {
    let mut mismatches = Vec::new();
    for c in all() {
        let (a, b) = (ours(c.runs), reference(c.input, c.dir));
        if a != b {
            mismatches.push(format!("{}: ours={a:?} unicode-bidi={b:?}", c.id));
        }
    }
    assert!(mismatches.is_empty(), "\n{}", mismatches.join("\n"));
}
