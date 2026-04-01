use std::cmp::max;
use std::fmt::Write;
use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit;

#[path = "renderer.generated.rs"]
mod renderer;

pub fn render(source: &str, cst: &SourceUnit) -> String {
    let mut w = String::new();

    write_source(&mut w, source);
    writeln!(&mut w).unwrap();

    writeln!(&mut w, "Tree:").unwrap();
    let mut ctx = RenderContext {
        source,
        w: &mut w,
        depth: 0,
    };
    ctx.write_indent();
    ctx.write_key("root", "SourceUnit");
    renderer::render_source_unit(&mut ctx, cst);

    w
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
        write!(self.w, "({label}: {kind})").unwrap();
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
        writeln!(
            w,
            "  {line_number: <2} \u{2502} {line}{padding} \u{2502} {range:?}",
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
