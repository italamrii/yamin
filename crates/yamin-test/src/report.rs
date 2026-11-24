//! JSON + Markdown reports for an evaluation run.

use crate::cases::{find, Category};
use crate::eval::{Report, Verdict};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Summary {
    pub pass: usize,
    pub fail: usize,
    pub skip: usize,
    pub total: usize,
}

impl Summary {
    pub fn evaluated(&self) -> usize {
        self.pass + self.fail
    }

    /// Pass rate over evaluated cases, in percent (0 if none evaluated).
    pub fn pass_pct(&self) -> u32 {
        match self.evaluated() {
            0 => 0,
            n => ((self.pass * 100) as f64 / n as f64).round() as u32,
        }
    }

    fn add(&mut self, v: Verdict) {
        self.total += 1;
        match v {
            Verdict::Pass => self.pass += 1,
            Verdict::Fail => self.fail += 1,
            Verdict::Skip => self.skip += 1,
        }
    }
}

pub fn summary(r: &Report) -> Summary {
    let mut s = Summary::default();
    for x in &r.results {
        s.add(x.verdict);
    }
    s
}

pub fn to_json(r: &Report) -> String {
    serde_json::to_string_pretty(r).expect("report serializes")
}

pub fn from_json(s: &str) -> serde_json::Result<Report> {
    serde_json::from_str(s)
}

fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Base file name for this report's .json/.md files.
pub fn file_stem(r: &Report) -> String {
    format!("yamin-{}-{}-{}", slug(&r.terminal), slug(&r.cli), r.date)
}

fn md_cell(s: &str) -> String {
    s.replace('|', "\\|").replace('`', "'")
}

pub fn to_markdown(r: &Report) -> String {
    let s = summary(r);
    let mut md = String::new();
    md.push_str("# yamin-test report\n\n");
    md.push_str("| | |\n|---|---|\n");
    md.push_str(&format!("| Terminal | {} |\n", md_cell(&r.terminal)));
    md.push_str(&format!("| AI CLI | {} |\n", md_cell(&r.cli)));
    md.push_str(&format!("| Date | {} |\n", r.date));
    md.push_str(&format!("| yamin-test | {} |\n\n", r.version));

    md.push_str(&format!(
        "**Result: {} / {} passed ({}%)** — {} failed, {} skipped, {} cases.\n\n",
        s.pass,
        s.evaluated(),
        s.pass_pct(),
        s.fail,
        s.skip,
        s.total
    ));

    md.push_str("## By category\n\n| Category | Pass | Fail | Skip |\n|---|---|---|---|\n");
    for cat in Category::ALL {
        let mut c = Summary::default();
        for x in r.results.iter().filter(|x| x.category == cat.slug()) {
            c.add(x.verdict);
        }
        if c.total > 0 {
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                cat.slug(),
                c.pass,
                c.fail,
                c.skip
            ));
        }
    }

    let fails: Vec<_> = r
        .results
        .iter()
        .filter(|x| x.verdict == Verdict::Fail)
        .collect();
    md.push_str("\n## Failures\n\n");
    if fails.is_empty() {
        md.push_str("None.\n");
    }
    for x in fails {
        let input = find(&x.id).map(|c| c.input).unwrap_or("");
        md.push_str(&format!("- **{}** `{}`", x.id, md_cell(input)));
        if let Some(note) = &x.note {
            md.push_str(&format!(" — {}", md_cell(note)));
        }
        md.push('\n');
    }

    md.push_str("\n## All cases\n\n| Case | Verdict | Note |\n|---|---|---|\n");
    for x in &r.results {
        let v = match x.verdict {
            Verdict::Pass => "✅ pass",
            Verdict::Fail => "❌ fail",
            Verdict::Skip => "⏭ skip",
        };
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            x.id,
            v,
            md_cell(x.note.as_deref().unwrap_or(""))
        ));
    }
    md
}

/// `YYYY-MM-DD` for a count of days since 1970-01-01 (proleptic Gregorian).
pub fn civil_date(days: i64) -> String {
    // Howard Hinnant's days_from_civil inverse.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{y:04}-{m:02}-{d:02}")
}

/// Today's UTC date.
pub fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    civil_date(secs.div_euclid(86_400))
}
