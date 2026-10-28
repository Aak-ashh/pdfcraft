use super::as_html::allowed;
use super::layout::{Cell, Item, MAX_COLS, Page, items};

/// Markdown's deepest heading.
const MAX_HEADING: u8 = 6;

/// `s` as Markdown text on one line: line breaks become spaces, control characters are dropped,
/// and every character Markdown would read as markup is escaped with a backslash.
fn md_text(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars().filter(|c| allowed(*c)) {
        match c {
            '\n' | '\r' | '\t' => o.push(' '),
            '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '#' | '|' | '~' | '!' | '&' => {
                o.push('\\');
                o.push(c);
            }
            c => o.push(c),
        }
    }
    o
}

/// `line` with its start escaped where Markdown would read a list item there (`- `, `+ `,
/// `1. `, `1) `); the other block markers (`#`, `>`) are escaped by [`md_text`] already.
fn block_start(line: String) -> String {
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    match line.as_bytes().get(digits) {
        Some(b'.' | b')') if digits > 0 => format!("{}\\{}", line.get(..digits).unwrap_or_default(), line.get(digits..).unwrap_or_default()),
        Some(b'-' | b'+') if digits == 0 => format!("\\{line}"),
        _ => line,
    }
}

/// `text` (already escaped) in bold and/or italic. The markers go around the text without its
/// outer spaces, which Markdown would not read as emphasis.
pub(crate) fn emphasis(text: &str, bold: bool, italic: bool) -> String {
    let core = text.trim();
    if core.is_empty() || !(bold || italic) {
        return text.to_string();
    }
    let lead = text.get(..text.len().saturating_sub(text.trim_start().len())).unwrap_or_default();
    let trail = text.get(text.trim_end().len()..).unwrap_or_default();
    let mark = match (bold, italic) {
        (true, true) => "***",
        (true, false) => "**",
        _ => "*",
    };
    format!("{lead}{mark}{core}{mark}{trail}")
}

fn cell(c: &Cell) -> String {
    emphasis(&md_text(&c.text), c.bold, c.italic)
}

/// Markdown (CommonMark, with GitHub's pipe tables): headings, paragraphs with bold and italic,
/// tables with their first row as the header, and a rule between pages. Images are left out.
pub fn markdown(pages: &[Page]) -> String {
    let mut blocks: Vec<String> = Vec::new();
    for it in items(pages) {
        match it {
            Item::Para(runs, lvl) => {
                // A heading's weight is the heading's own, as in the HTML export.
                let text: String = runs.iter().map(|r| emphasis(&md_text(&r.text), r.block.bold && lvl == 0, r.block.italic)).collect();
                let text = text.trim();
                if text.is_empty() {
                    continue;
                }
                blocks.push(match lvl {
                    0 => block_start(text.to_string()),
                    l => format!("{} {text}", "#".repeat(usize::from(l.min(MAX_HEADING)))),
                });
            }
            Item::Table(t) => {
                // Each row as wide as the widest; a cell spanning columns is followed by empty ones.
                // Detected tables have at most MAX_COLS columns, and a span never counts for more.
                let span = |c: &Cell| c.span.clamp(1, MAX_COLS);
                let width = t.rows.iter().map(|r| r.iter().map(span).fold(0usize, usize::saturating_add)).max().unwrap_or(0);
                if width == 0 {
                    continue;
                }
                let mut lines = Vec::with_capacity(t.rows.len().saturating_add(1));
                for (i, row) in t.rows.iter().enumerate() {
                    let mut cells: Vec<String> = Vec::with_capacity(width);
                    for c in row {
                        cells.push(cell(c));
                        cells.extend(std::iter::repeat_n(String::new(), span(c).saturating_sub(1)));
                    }
                    cells.resize(width, String::new());
                    lines.push(format!("| {} |", cells.join(" | ")));
                    if i == 0 {
                        lines.push(format!("|{}", " --- |".repeat(width)));
                    }
                }
                blocks.push(lines.join("\n"));
            }
            Item::Img(_) => {}
            Item::PageBreak => blocks.push("---".into()),
        }
    }
    let mut s = blocks.join("\n\n");
    s.push('\n');
    s
}
