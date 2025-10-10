use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Write;
use std::ops::Range;
use std::sync::LazyLock;

use anyhow::Result;
use inflector::Inflector;
use language_definition::model::Item;
use slang_solidity::cst::{Cursor, Node, NonterminalKind, TextRangeExtensions};
use solidity_language::SolidityDefinition;

pub fn render(source: &str, errors: &Vec<String>, cursor: &mut Cursor) -> Result<String> {
    let mut w = String::new();

    write_source(&mut w, source)?;
    writeln!(&mut w)?;

    write_errors(&mut w, errors)?;
    writeln!(&mut w)?;

    write_tree(&mut w, cursor, source)?;

    Ok(w)
}

fn write_source(w: &mut String, source: &str) -> Result<()> {
    if source.is_empty() {
        writeln!(w, "Source: \"\"")?;
        return Ok(());
    }

    // "lines()" only handles "\n", so let's normalize all line endings:
    let source = source.replace("\r\n", "\n").replace('\r', "\n");

    let line_data = source
        .lines()
        .enumerate()
        .map(|(index, line)| (index, line, line.len(), line.chars().count()))
        .collect::<Vec<_>>();

    let source_width = {
        let source_width = line_data
            .iter()
            .map(|(_, _, _, chars)| *chars)
            .max()
            .unwrap_or(0);

        max(source_width, 80)
    };

    writeln!(w, "Source: >")?;

    let mut offset = 0;
    for (index, line, bytes, chars) in &line_data {
        let range = offset..(offset + bytes);
        writeln!(
            w,
            "  {line_number: <2} │ {line}{padding} │ {range:?}",
            line_number = index + 1,
            padding = " ".repeat(source_width - chars),
        )?;

        offset = range.end + 1;
    }

    Ok(())
}

fn write_errors(w: &mut String, errors: &Vec<String>) -> Result<()> {
    if errors.is_empty() {
        writeln!(w, "Errors: []")?;
        return Ok(());
    }

    writeln!(w, "Errors: # {count} total", count = errors.len())?;

    for error in errors {
        writeln!(w, "  - >")?;
        for line in error.lines() {
            writeln!(w, "    {line}")?;
        }
    }

    Ok(())
}

fn write_tree(w: &mut String, cursor: &mut Cursor, source: &str) -> Result<()> {
    writeln!(w, "Tree:")?;
    write_node(w, cursor, source, 0)?;

    assert!(!cursor.go_to_next());
    Ok(())
}

fn write_node(w: &mut String, cursor: &mut Cursor, source: &str, depth: usize) -> Result<()> {
    let indentation = " ".repeat(4 * depth);
    write!(w, "{indentation}  - ")?;

    loop {
        write!(w, "{key}", key = render_key(cursor))?;

        // If it is a parent wih a single child, inline them on the same line:
        if matches!(cursor.node(), Node::Nonterminal(nonterminal) if nonterminal.children.len() == 1 && !NON_INLINABLE.contains(&nonterminal.kind))
        {
            let parent_range = cursor.text_range();
            assert!(cursor.go_to_next());

            let child_range = cursor.text_range();
            assert_eq!(parent_range, child_range);

            write!(w, " ► ")?;
            continue;
        }

        break;
    }

    writeln!(w, ": {value}", value = render_value(cursor, source))?;

    for _ in cursor.node().children() {
        assert!(cursor.go_to_next());
        write_node(w, cursor, source, depth + 1)?;
    }

    Ok(())
}

fn render_key(cursor: &mut Cursor) -> String {
    let kind = match cursor.node() {
        Node::Nonterminal(nonterminal) => nonterminal.kind.to_string(),
        Node::Terminal(terminal) => terminal.kind.to_string(),
    };

    format!(
        "({label}꞉ {kind})",
        label = cursor.label().as_ref().to_snake_case()
    )
}

fn render_value(cursor: &mut Cursor, source: &str) -> String {
    let utf8_range = cursor.text_range().utf8();
    let char_range = {
        let start = source[..utf8_range.start].chars().count();
        let end = source[..utf8_range.end].chars().count();
        start..end
    };
    let preview = render_preview(source, &char_range);

    match cursor.node() {
        Node::Nonterminal(nonterminal) if nonterminal.children.is_empty() => {
            format!("[] # ({utf8_range:?})")
        }
        Node::Nonterminal(_) => format!("# {preview} ({utf8_range:?})"),
        Node::Terminal(_) => format!("{preview} # ({utf8_range:?})"),
    }
}

fn render_preview(source: &str, char_range: &Range<usize>) -> String {
    let length = char_range.len();

    // Trim long values:
    let max_length = 50;
    let mut contents: String = source
        .chars()
        .skip(char_range.start)
        .take(min(length, max_length))
        .collect();

    // Add terminator if trimmed:
    if length > max_length {
        contents.push_str("...");
    }

    // Escape line breaks:
    let contents = contents
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n");

    // Surround by quotes for use in yaml:
    if contents.contains('"') {
        let contents = contents.replace('\'', "''");
        format!("'{contents}'")
    } else {
        let contents = contents.replace('"', "\\\"");
        format!("\"{contents}\"")
    }
}

static NON_INLINABLE: LazyLock<HashSet<NonterminalKind>> = LazyLock::new(|| {
    let mut kinds = HashSet::new();

    for item in SolidityDefinition::create().items() {
        match item {
            Item::Repeated { .. } | Item::Separated { .. } => {
                // Do not inline these parents, even if they have a single child.
                kinds.insert(item.name().parse().unwrap());
            }
            Item::Struct { .. } | Item::Enum { .. } | Item::Precedence { .. } => {
                // These nonterminals can be inlined if they have a single child.
                // Note: same goes for 'PrecedenceExpression' items under each 'Precedence' item.
            }
            Item::Trivia { .. }
            | Item::Keyword { .. }
            | Item::Token { .. }
            | Item::Fragment { .. } => {
                // These are terminals (no children).
            }
        }
    }

    kinds
});
