use yamin_test::shaping::{shape, LAM_ALEF_FILLER};

fn cps(clusters: &[String]) -> Vec<u32> {
    clusters
        .iter()
        .flat_map(|c| c.chars())
        .map(|c| c as u32)
        .collect()
}

#[test]
fn shapes_marhaba_with_positional_forms() {
    // م initial, ر final, ح initial (ر does not join left), ب medial, ا final
    assert_eq!(
        cps(&shape("مرحبا")),
        vec![0xFEE3, 0xFEAE, 0xFEA3, 0xFE92, 0xFE8E]
    );
}

#[test]
fn isolated_letter() {
    assert_eq!(cps(&shape("ب")), vec![0xFE8F]);
}

#[test]
fn lam_alef_isolated_occupies_two_cells() {
    // D1: ligature + filler so the cell count equals the char count.
    let s = shape("لا");
    assert_eq!(s.len(), 2);
    assert_eq!(cps(&s), vec![0xFEFB, LAM_ALEF_FILLER as u32]);
}

#[test]
fn lam_alef_final_after_joining_letter() {
    // س initial, لا final, filler, م isolated (alef does not join left)
    assert_eq!(
        cps(&shape("سلام")),
        vec![0xFEB3, 0xFEFC, LAM_ALEF_FILLER as u32, 0xFEE1]
    );
}

#[test]
fn lam_alef_variants() {
    assert_eq!(cps(&shape("لأ"))[0], 0xFEF7);
    assert_eq!(cps(&shape("لإ"))[0], 0xFEF9);
    assert_eq!(cps(&shape("لآ"))[0], 0xFEF5);
}

#[test]
fn harakat_stay_in_their_base_cluster_and_are_transparent() {
    // ب initial + kasra, س medial + sukun, م final + kasra
    let s = shape("بِسْمِ");
    assert_eq!(s.len(), 3);
    assert_eq!(
        cps(&s),
        vec![0xFE91, 0x0650, 0xFEB4, 0x0652, 0xFEE2, 0x0650]
    );
}

#[test]
fn non_arabic_passes_through() {
    assert_eq!(shape("a1 ؟"), vec!["a", "1", " ", "؟"]);
}
