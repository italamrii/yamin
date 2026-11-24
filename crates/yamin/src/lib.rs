//! yamin: BiDi + Arabic shaping for terminal cell grids.
//!
//! Implements the decisions in `docs/spec.md` (freedesktop Terminal BiDi
//! recommendation + UAX#9). The model is a paragraph of cells in the order the
//! application wrote them; [`layout`] returns, for each row, the cells in
//! visual left-to-right order with the glyph to draw.

use unicode_bidi::{bidi_class, BidiClass};

/// One terminal cell in the model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    /// A base character followed by its combining marks (D4).
    Text(String),
    /// Never written / erased. Ignored by BiDi and placed at the end of the row.
    Erased,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Ltr,
    Rtl,
}

/// BDSM: implicit (`CSI 8 h`, default) or explicit (`CSI 8 l`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Implicit,
    Explicit,
}

/// Per-paragraph BiDi properties (spec section 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Props {
    pub mode: Mode,
    /// SCP direction; the fallback when autodetection finds no strong char.
    pub direction: Direction,
    /// `CSI ? 2501 h`. Off by default (D5). Ignored in explicit mode.
    pub autodetect: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props { mode: Mode::Implicit, direction: Direction::Ltr, autodetect: false }
    }
}

/// What to draw in a visual cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Glyph {
    /// Base glyph (shaped and mirrored as needed) followed by its marks.
    Text(String),
    /// Second cell of a lam-alef ligature (D1); the ligature is drawn by the
    /// visually adjacent lam cell and covers both cells.
    LigatureTail,
    Erased,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualCell {
    /// Index of the model cell in the paragraph.
    pub logical: usize,
    pub glyph: Glyph,
    /// Resolved RTL (odd level). Decides the cursor side (spec section 2).
    pub rtl: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    /// Resolved paragraph direction.
    pub direction: Direction,
    /// Rows of the paragraph, each in visual left-to-right order.
    pub rows: Vec<Vec<VisualCell>>,
}

impl Layout {
    pub fn visual_to_logical(&self, row: usize) -> Vec<usize> {
        self.rows[row].iter().map(|v| v.logical).collect()
    }

    /// Indexed by the cell's position within the row (logical - row start).
    pub fn logical_to_visual(&self, row: usize) -> Vec<usize> {
        let cells = &self.rows[row];
        let start = cells.iter().map(|v| v.logical).min().unwrap_or(0);
        let mut out = vec![0; cells.len()];
        for (vis, v) in cells.iter().enumerate() {
            out[v.logical - start] = vis;
        }
        out
    }

    /// The row as text: lam-alef tails and erased cells become spaces.
    pub fn row_text(&self, row: usize) -> String {
        self.rows[row]
            .iter()
            .map(|v| match &v.glyph {
                Glyph::Text(t) => t.as_str(),
                Glyph::LigatureTail | Glyph::Erased => " ",
            })
            .collect()
    }
}

/// Split text into cells: each base char with the combining marks after it.
pub fn cells_from_str(s: &str) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
    for c in s.chars() {
        if bidi_class(c) == BidiClass::NSM {
            if let Some(Cell::Text(t)) = cells.last_mut() {
                t.push(c);
                continue;
            }
        }
        cells.push(Cell::Text(c.to_string()));
    }
    cells
}

/// Lay out one paragraph soft-wrapped at `width` cells per row.
pub fn layout(cells: &[Cell], width: usize, props: Props) -> Layout {
    let width = width.max(1);
    let ranges = (0..cells.len()).step_by(width).map(|s| s..(s + width).min(cells.len()));
    match props.mode {
        Mode::Implicit => {
            let direction = if props.autodetect {
                detect(cells).unwrap_or(props.direction)
            } else {
                props.direction
            };
            let glyphs = shaping::shape(cells, width);
            let rows = bidi::implicit_rows(cells, &glyphs, ranges, direction);
            Layout { direction, rows }
        }
        Mode::Explicit => {
            let direction = props.direction;
            Layout { direction, rows: ranges.map(|r| explicit_row(cells, r, direction)).collect() }
        }
    }
}

/// P2/P3 over the whole paragraph: the first strong character decides.
/// `None` when there is none, so the caller falls back to the SCP direction.
fn detect(cells: &[Cell]) -> Option<Direction> {
    cells
        .iter()
        .filter_map(|c| match c {
            Cell::Text(t) => Some(t.chars()),
            Cell::Erased => None,
        })
        .flatten()
        .find_map(|c| match bidi_class(c) {
            BidiClass::L => Some(Direction::Ltr),
            BidiClass::R | BidiClass::AL => Some(Direction::Rtl),
            _ => None,
        })
}

/// Explicit mode: the model is already visual. LTR draws it as-is; RTL draws
/// the row reversed and mirrors mirrorable glyphs. No shaping.
fn explicit_row(cells: &[Cell], row: std::ops::Range<usize>, direction: Direction) -> Vec<VisualCell> {
    let rtl = direction == Direction::Rtl;
    let mut out: Vec<VisualCell> = row
        .map(|i| {
            let glyph = match &cells[i] {
                Cell::Text(t) => Glyph::Text(t.clone()),
                Cell::Erased => Glyph::Erased,
            };
            VisualCell { logical: i, glyph: mirrored(glyph, rtl), rtl }
        })
        .collect();
    if rtl {
        out.reverse();
    }
    out
}

mod shaping;

/// UAX#9 L4: a cell resolved RTL draws the Bidi_Mirroring_Glyph of its base.
fn mirrored(glyph: Glyph, rtl: bool) -> Glyph {
    match glyph {
        Glyph::Text(t) if rtl => {
            let mut chars = t.chars();
            match chars.next().and_then(unicode_bidi_mirroring::get_mirrored) {
                Some(m) => Glyph::Text(std::iter::once(m).chain(chars).collect()),
                None => Glyph::Text(t),
            }
        }
        other => other,
    }
}

mod bidi {
    use super::{mirrored, Cell, Direction, Glyph, VisualCell};
    use std::ops::Range;
    use unicode_bidi::{bidi_class, BidiClass, BidiInfo, Level};

    /// The paragraph text handed to UAX#9, with each cell's byte range.
    struct ParaText {
        text: String,
        /// Byte range of each cell in `text`; `None` for erased cells.
        span: Vec<Option<Range<usize>>>,
    }

    impl ParaText {
        fn new(cells: &[Cell]) -> Self {
            let mut text = String::new();
            let mut span = Vec::with_capacity(cells.len());
            for cell in cells {
                match cell {
                    Cell::Text(t) => {
                        let start = text.len();
                        for c in t.chars() {
                            // Cells never end a paragraph: neutralize separators so
                            // UAX#9 sees exactly one paragraph (P1).
                            text.push(if bidi_class(c) == BidiClass::B { '\u{FFFD}' } else { c });
                        }
                        span.push(Some(start..text.len()));
                    }
                    Cell::Erased => span.push(None),
                }
            }
            ParaText { text, span }
        }

        fn start(&self, i: usize) -> usize {
            self.span[i].as_ref().expect("text cell").start
        }
    }

    /// Implicit mode: levels resolved on the whole paragraph, reordering per
    /// row (UAX#9 section 3: lines are reordered after levels are resolved).
    pub(super) fn implicit_rows(
        cells: &[Cell],
        glyphs: &[Glyph],
        rows: impl Iterator<Item = Range<usize>>,
        direction: Direction,
    ) -> Vec<Vec<VisualCell>> {
        let para = ParaText::new(cells);
        let level = match direction {
            Direction::Ltr => Level::ltr(),
            Direction::Rtl => Level::rtl(),
        };
        let info = (!para.text.is_empty()).then(|| BidiInfo::new(&para.text, Some(level))); // HL1
        let rtl_para = direction == Direction::Rtl;

        rows.map(|row| {
            let text_cells: Vec<usize> = row.clone().filter(|&i| para.span[i].is_some()).collect();
            let erased = row.filter(|&i| para.span[i].is_none()).map(|i| VisualCell {
                logical: i,
                glyph: Glyph::Erased,
                rtl: rtl_para,
            });

            let mut visual = Vec::with_capacity(cells.len());
            if let (Some(info), Some(&first), Some(&last)) = (&info, text_cells.first(), text_cells.last()) {
                let line = para.start(first)..para.span[last].as_ref().expect("text cell").end;
                let paragraph = &info.paragraphs[0];
                // L1 + L2 for this line; runs come back in visual order.
                let (levels, runs) = info.visual_runs(paragraph, line);
                for run in runs {
                    let mut in_run: Vec<usize> = text_cells
                        .iter()
                        .copied()
                        .filter(|&i| run.contains(&para.start(i)))
                        .collect();
                    if levels[run.start].is_rtl() {
                        in_run.reverse();
                    }
                    visual.extend(in_run.into_iter().map(|i| {
                        let rtl = levels[para.start(i)].is_rtl();
                        VisualCell { logical: i, glyph: mirrored(glyphs[i].clone(), rtl), rtl }
                    }));
                }
            }

            // Erased cells go to the end of the row in the paragraph direction.
            if rtl_para {
                let mut e: Vec<VisualCell> = erased.collect();
                e.reverse();
                e.extend(visual);
                e
            } else {
                visual.extend(erased);
                visual
            }
        })
        .collect()
    }
}
