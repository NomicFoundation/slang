//! Renders V2 CST parse results into a human-readable, YAML snapshot format.
//!
//! ## Output format
//!
//! - **`Source:`** — annotated source lines with byte ranges.
//! - **`Error:`** (failure) — rendered diagnostic output.
//! - **`Tree:`** (successful parser) — nested CST structure with inline previews and byte ranges.
//!
//! ## Symbol legend
//!
//! - `꞉` (U+A789, MODIFIER LETTER COLON) — separates label from kind in node
//!   entries, used instead of `:` (regular colon) to avoid YAML key-value syntax conflicts.
//! - `►` (U+25BA, BLACK RIGHT-POINTING POINTER) — marks the selected variant of
//!   a choice node, inlined on the same line as its parent.
//! - `│` (U+2502, BOX DRAWINGS LIGHT VERTICAL) — column border in the `Source:`
//!   section, used instead of `|` to avoid confusion with Solidity's bitwise OR.

use std::cmp::max;
use std::fmt::Write;
use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::validation::SyntaxVersionError;
use slang_solidity_v2_parser::ParseOutput;

use crate::reporting::diagnostic;

#[path = "renderer.generated.rs"]
mod renderer;

/// The output of a render function: an optional byte range and a vec of string
/// fragments that are concatenated directly (no separators) to form the output.
///
/// We return the byte range since it's not included in the non terminal nodes.
/// It's optional since a non terminal may not be present (like an empty sequence).
pub(crate) type RenderedOutput = (Option<Range<usize>>, Vec<String>);

/// Format a label-kind pair for YAML output.
///
/// Note: \u{a789} (MODIFIER LETTER COLON) is used instead of : to avoid
/// conflicting with YAML's key-value syntax, which breaks YAML linters.
pub(crate) fn format_label_kind(label: &str, kind: &str) -> String {
    format!("({label}\u{a789} {kind})")
}

/// Render a parse result (success or failure) to YAML format.
///
/// Returns a tuple of (`is_success`, `rendered_output`), where `is_success` is true if
/// `result` had no errors.
pub fn render(
    source: &str,
    source_id: &str,
    result: &(ParseOutput, Vec<SyntaxVersionError>),
) -> (bool, String) {
    let mut w = String::new();

    // Write the source code
    write_source(&mut w, source);
    writeln!(&mut w).unwrap();

    // Write the errors
    let (
        ParseOutput {
            source_unit,
            errors,
        },
        validation_errors,
    ) = result;

    // TODO(v2): this will move to the `Diagnostic` reporter API:
    let mut all_errors = Vec::new();
    all_errors.extend(
        errors
            .iter()
            .map(|e| diagnostic::render(e, source_id, source, false)),
    );
    all_errors.extend(
        validation_errors
            .iter()
            .map(|e| diagnostic::render(e, source_id, source, false)),
    );

    let is_success = !write_errors(&mut w, &all_errors).unwrap();

    // Write the Tree
    writeln!(&mut w, "Tree:").unwrap();
    let (_, root_frags) = renderer::render_source_unit(source, source_unit, 0);
    write!(w, "  - {}", format_label_kind("root", "SourceUnit")).unwrap();
    for frag in root_frags {
        w.push_str(&frag);
    }

    (is_success, w)
}

/// Helper to accumulate rendered children, merge their ranges,
/// and produce the final `RenderedOutput` for a nonterminal node.
pub(crate) struct ChildrenAccumulator<'a> {
    source: &'a str,
    depth: usize,
    range_start: usize,
    range_end: usize,
    fragments: Vec<String>,
}

impl<'a> ChildrenAccumulator<'a> {
    pub fn new(source: &'a str, depth: usize) -> Self {
        Self {
            source,
            depth,
            range_start: usize::MAX,
            range_end: 0,
            fragments: Vec::new(),
        }
    }

    pub fn add(&mut self, label: &str, kind: &str, (range, child_frags): RenderedOutput) {
        if let Some(ref r) = range {
            self.range_start = self.range_start.min(r.start);
            self.range_end = self.range_end.max(r.end);
        }
        let indent = " ".repeat(4 * self.depth);
        self.fragments
            .push(format!("{indent}  - {}", format_label_kind(label, kind)));
        self.fragments.extend(child_frags);
    }

    pub fn finish(self) -> RenderedOutput {
        let range = if self.range_start <= self.range_end {
            Some(self.range_start..self.range_end)
        } else {
            None
        };

        let value = if self.fragments.is_empty() {
            ": []\n".to_string()
        } else if let Some(ref r) = range {
            let preview = render_preview(self.source, r);
            format!(": # {preview} ({}..{})\n", r.start, r.end)
        } else {
            ":\n".to_string()
        };

        let mut fragments = self.fragments;
        fragments.insert(0, value);
        (range, fragments)
    }
}

/// Produce the `RenderedOutput` for a terminal node.
pub(crate) fn render_terminal(source: &str, range: &Range<usize>) -> RenderedOutput {
    let preview = render_preview(source, range);
    (
        Some(range.clone()),
        vec![format!(": {preview} # ({}..{})\n", range.start, range.end)],
    )
}

fn write_source(w: &mut String, source: &str) {
    if source.is_empty() {
        writeln!(w, "Source: \"\"").unwrap();
        return;
    }

    let source = source.replace("\r\n", "\n").replace('\r', "\n");

    let line_data: Vec<_> = source
        .lines()
        .enumerate()
        .map(|(index, line)| (index, line, line.len(), line.chars().count()))
        .collect();

    let source_width = {
        let w = line_data
            .iter()
            .map(|(_, _, _, chars)| *chars)
            .max()
            .unwrap_or(0);
        max(w, 80)
    };

    writeln!(w, "Source: >").unwrap();

    let mut offset = 0;
    for (index, line, bytes, chars) in &line_data {
        let range = offset..(offset + bytes);
        // Note: │ (U+2502, BOX DRAWINGS LIGHT VERTICAL) is used instead of |
        // to avoid confusion with Solidity's bitwise OR operator in source code.
        writeln!(
            w,
            "  {line_number: <2} │ {line}{padding} │ {range:?}",
            line_number = index + 1,
            padding = " ".repeat(source_width - chars),
        )
        .unwrap();

        offset = range.end + 1;
    }
}

fn render_preview(source: &str, range: &Range<usize>) -> String {
    let text = source.get(range.start..range.end).unwrap_or_else(|| {
        panic!(
            "render_preview: byte range {}..{} out of bounds for source of length {}",
            range.start,
            range.end,
            source.len()
        )
    });
    let char_count = text.chars().count();

    let max_length = 50;
    let mut contents: String = text.chars().take(max_length).collect();

    if char_count > max_length {
        contents.push_str("...");
    }

    // Escape line breaks
    let contents = contents
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n");

    // Quote for YAML
    if contents.contains('"') {
        let contents = contents.replace('\'', "''");
        format!("'{contents}'")
    } else {
        format!("\"{contents}\"")
    }
}

fn write_errors(w: &mut String, errors: &Vec<String>) -> Result<bool, std::fmt::Error> {
    if errors.is_empty() {
        return Ok(false);
    }

    writeln!(w, "Errors: # {count} total", count = errors.len())?;

    for error in errors {
        writeln!(w, "  - >")?;
        for line in error.lines() {
            writeln!(w, "    {line}")?;
        }
    }

    Ok(true)
}
