//! Results matrix: terminals (rows) × AI CLI tools (columns), from reports.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::eval::Report;
use crate::report::{from_json, summary};

fn cell(r: &Report) -> String {
    let s = summary(r);
    let mut c = format!("{}/{} ({}%)", s.pass, s.evaluated(), s.pass_pct());
    if s.skip > 0 {
        c.push_str(&format!(" +{} skipped", s.skip));
    }
    c
}

/// Column order: "none" (bare shell) first, then alphabetical.
fn cli_order(a: &&str, b: &&str) -> std::cmp::Ordering {
    (*a != "none", a.to_lowercase()).cmp(&(*b != "none", b.to_lowercase()))
}

pub fn build(reports: &[Report]) -> String {
    if reports.is_empty() {
        return "No reports found. Run `yamin-test eval` first.\n".into();
    }
    // Latest report per (terminal, cli) pair.
    let mut latest: BTreeMap<(&str, &str), &Report> = BTreeMap::new();
    for r in reports {
        let key = (r.terminal.as_str(), r.cli.as_str());
        if latest.get(&key).is_none_or(|old| r.date > old.date) {
            latest.insert(key, r);
        }
    }
    let terminals: BTreeSet<&str> = latest.keys().map(|k| k.0).collect();
    let mut clis: Vec<&str> = latest.keys().map(|k| k.1).collect::<BTreeSet<_>>().into_iter().collect();
    clis.sort_by(cli_order);

    let mut md = String::from("# yamin-test results matrix\n\n");
    md.push_str("Cells: passed / evaluated (pass rate). Skipped cases are not counted.\n\n");
    md.push_str(&format!("| Terminal | {} |\n", clis.join(" | ")));
    md.push_str(&format!("|---|{}\n", "---|".repeat(clis.len())));
    for t in terminals {
        let cells: Vec<String> = clis
            .iter()
            .map(|c| latest.get(&(t, *c)).map_or("—".to_string(), |r| cell(r)))
            .collect();
        md.push_str(&format!("| {} | {} |\n", t, cells.join(" | ")));
    }
    md
}

/// Load every `*.json` report in `dir`.
pub fn load_dir(dir: &Path) -> Result<Vec<Report>, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut reports = Vec::new();
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        reports.push(from_json(&text).map_err(|e| format!("{}: {e}", path.display()))?);
    }
    Ok(reports)
}
