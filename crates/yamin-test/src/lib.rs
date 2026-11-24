//! yamin-test: a conformance catalog for Arabic BiDi + shaping in terminals.
//!
//! The expected forms are derived from `docs/spec.md`. The shaping table here
//! is a *test oracle*, deliberately independent from the future `yamin` library.

pub mod cases;
pub mod display;
pub mod shaping;
pub mod html;
pub mod terminal;
pub mod eval;
pub mod report;
