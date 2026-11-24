use yamin_test::cases::all;
use yamin_test::html::reference_page;

#[test]
fn reference_page_has_every_case_with_its_direction() {
    let page = reference_page();
    assert!(page.starts_with("<!doctype html>"));
    for c in all() {
        assert!(page.contains(&format!("id=\"{}\"", c.id)), "{}", c.id);
    }
    assert!(page.contains("dir=\"rtl\""));
    assert!(page.contains("dir=\"ltr\""));
}

#[test]
fn reference_page_escapes_html() {
    // code-05 contains double quotes; paths-03 contains backslashes.
    let page = reference_page();
    assert!(page.contains("let name = &quot;"));
}
