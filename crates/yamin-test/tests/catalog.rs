use std::collections::{BTreeMap, HashSet};
use yamin_test::cases::{all, Category, Run};

#[test]
fn catalog_has_40_cases() {
    assert_eq!(all().len(), 40);
}

#[test]
fn each_category_has_five_cases() {
    for cat in Category::ALL {
        let n = all().iter().filter(|c| c.category == cat).count();
        assert_eq!(n, 5, "category {:?}", cat);
    }
}

#[test]
fn ids_are_unique_and_prefixed_by_category() {
    let mut seen = HashSet::new();
    for c in all() {
        assert!(seen.insert(c.id), "duplicate id {}", c.id);
        assert!(c.id.starts_with(c.category.slug()), "{} not prefixed", c.id);
    }
}

#[test]
fn every_case_cites_rules_and_a_description() {
    for c in all() {
        assert!(!c.rules.is_empty(), "{} has no rules", c.id);
        assert!(!c.description.trim().is_empty(), "{} has no description", c.id);
    }
}

fn multiset(s: &str) -> BTreeMap<char, usize> {
    let mut m = BTreeMap::new();
    for ch in s.chars() {
        *m.entry(ch).or_insert(0) += 1;
    }
    m
}

/// The expected visual runs must be a permutation of the logical input:
/// reordering may move characters, never add or drop them.
#[test]
fn runs_are_a_permutation_of_the_input() {
    for c in all() {
        let joined: String = c
            .runs
            .iter()
            .map(|r| match r {
                Run::Ltr(t) | Run::Rtl(t) => *t,
            })
            .collect();
        assert_eq!(multiset(&joined), multiset(c.input), "case {}", c.id);
    }
}

#[test]
fn category_slugs_round_trip() {
    for cat in Category::ALL {
        assert_eq!(Category::from_slug(cat.slug()), Some(cat));
    }
    assert_eq!(Category::from_slug("nope"), None);
}
