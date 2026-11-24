//! yamin against the 40 yamin-test cases: the implicit-mode row must equal the
//! case's expected visual line (order, shaping, lam-alef D1, mirroring).

use yamin::{cells_from_str, layout, Direction, Props};
use yamin_test::cases::{all, Dir};
use yamin_test::display::expected_visual;

#[test]
fn all_40_cases_match_the_expected_visual_line() {
    let mut failures = Vec::new();
    for case in all() {
        let cells = cells_from_str(case.input);
        let direction = match case.dir {
            Dir::Ltr => Direction::Ltr,
            Dir::Rtl => Direction::Rtl,
        };
        let l = layout(&cells, cells.len(), Props { direction, ..Props::default() });
        let (got, want) = (l.row_text(0), expected_visual(case));
        if got != want {
            failures.push(format!("{}: got {got:?}, want {want:?}", case.id));
        }
    }
    assert!(failures.is_empty(), "{} of 40 failed:\n{}", failures.len(), failures.join("\n"));
}
