use std::cmp::max;
use std::fmt::Write;
use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit;
use slang_solidity_v2_cst::structured_cst::validation::SyntaxVersionError;
use slang_solidity_v2_parser::ParserError;

use crate::reporting::diagnostic;

#[path = "renderer.generated.rs"]
mod renderer;

/// Render a parse result (success or failure) to YAML format.
///
/// Returns `(status, content)` where status is `"success"` or `"failure"`.
pub fn render(
    source: &str,
    source_id: &str,
    result: &(Result<SourceUnit, ParserError>, Vec<SyntaxVersionError>),
) -> (&'static str, String) {
    let mut w = String::new();

    write_source(&mut w, source);
    writeln!(&mut w).unwrap();

    match result {
        (Ok(cst), validation_errors) if validation_errors.is_empty() => {
            writeln!(&mut w, "Tree:").unwrap();
            let mut ctx = RenderContext {
                source,
                w: &mut w,
                depth: 0,
            };
            ctx.write_indent();
            ctx.write_key("root", "SourceUnit");
            renderer::render_source_unit(&mut ctx, cst);

            ("success", w)
        }
        (Ok(cst), validation_errors) => {
            // Print validation errors, followed by the structured CST:
            for err in validation_errors {
                let rendered = diagnostic::render(err, &source_id, &source, false);
                writeln!(&mut w, "{rendered}").unwrap();
            }
            
            writeln!(&mut w, "Tree:").unwrap();
            let mut ctx = RenderContext {
                source,
                w: &mut w,
                depth: 0,
            };
            ctx.write_indent();
            ctx.write_key("root", "SourceUnit");
            renderer::render_source_unit(&mut ctx, cst);

            ("failure", w)
        }
        (Err(err), _) => {
            let rendered = diagnostic::render(err, source_id, source, false);
            writeln!(&mut w, "Error: >").unwrap();
            for line in rendered.lines() {
                writeln!(&mut w, "  {line}").unwrap();
            }

            ("failure", w)
        }
    }
}

pub(crate) struct RenderContext<'a> {
    pub source: &'a str,
    pub w: &'a mut String,
    pub depth: usize,
}

impl RenderContext<'_> {
    pub fn write_indent(&mut self) {
        let indentation = " ".repeat(4 * self.depth);
        write!(self.w, "{indentation}  - ").unwrap();
    }

    pub fn write_key(&mut self, label: &str, kind: &str) {
        // Note: \u{a789} (MODIFIER LETTER COLON) is used instead of : to avoid
        // conflicting with YAML's key-value syntax, which breaks YAML linters.
        write!(self.w, "({label}\u{a789} {kind})").unwrap();
    }

    pub fn write_connector(&mut self) {
        write!(self.w, " ► ").unwrap();
    }

    pub fn write_terminal_value(&mut self, range: &Range<usize>) {
        let preview = render_preview(self.source, range);
        writeln!(
            self.w,
            ": {preview} # ({start}..{end})",
            start = range.start,
            end = range.end
        )
        .unwrap();
    }

    pub fn write_nonterminal_start(&mut self) {
        writeln!(self.w, ":").unwrap();
    }

    pub fn write_empty_collection(&mut self) {
        writeln!(self.w, ": []").unwrap();
    }
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
    let text = &source[range.start..range.end];
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
