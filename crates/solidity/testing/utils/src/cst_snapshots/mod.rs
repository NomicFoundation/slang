use std::cmp::max;
use std::fmt::Write;

use anyhow::Result;
use inflector::Inflector;
use slang_solidity::cst::Node;
use slang_solidity::cursor::CursorWithNames;
use slang_solidity::text_index::TextRangeExtensions;

pub struct CstSnapshots;

impl CstSnapshots {
    pub fn render(source: &str, errors: &Vec<String>, cursor: CursorWithNames) -> Result<String> {
        let mut w = String::new();

        write_source(&mut w, source)?;
        writeln!(&mut w)?;

        write_errors(&mut w, errors)?;
        writeln!(&mut w)?;

        write_tree(&mut w, cursor, source)?;

        Ok(w)
    }
}

fn write_source<W: Write>(w: &mut W, source: &str) -> Result<()> {
    if source.is_empty() {
        writeln!(w, "Source: \"\"")?;
        return Ok(());
    }

    let line_data = source
        .lines()
        .enumerate()
        .map(|(index, line)| (index, line, line.bytes().len(), line.chars().count()))
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

fn write_errors<W: Write>(w: &mut W, errors: &Vec<String>) -> Result<()> {
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

fn write_tree<W: Write>(w: &mut W, mut cursor: CursorWithNames, source: &str) -> Result<()> {
    writeln!(w, "Tree:")?;
    write_node(w, &mut cursor, source, 0)?;

    assert!(!cursor.go_to_next());
    Ok(())
}

fn write_node<W: Write>(
    w: &mut W,
    cursor: &mut CursorWithNames,
    source: &str,
    depth: usize,
) -> Result<()> {
    let indentation = " ".repeat(4 * depth);
    write!(w, "{indentation}  - ")?;

    loop {
        let key = render_key(cursor);
        write!(w, "{key}")?;

        // combine parents with a single child on the same line:
        if cursor.node().children().len() == 1 {
            let parent_range = cursor.text_range();
            assert!(cursor.go_to_next());

            let child_range = cursor.text_range();
            assert_eq!(parent_range, child_range);

            write!(w, " ► ")?;
            continue;
        }

        break;
    }

    let value = render_value(cursor, source)?;
    if value.is_empty() {
        write!(w, ":")?;
    } else {
        write!(w, ": {value}")?;
    }

    let comment = render_comment(cursor, source)?;
    writeln!(w, " # {comment}")?;

    for _ in cursor.node().children() {
        assert!(cursor.go_to_next());
        write_node(w, cursor, source, depth + 1)?;
    }

    Ok(())
}

fn render_key(cursor: &mut CursorWithNames) -> String {
    let kind = match cursor.node() {
        Node::Rule(rule) => rule.kind.to_string(),
        Node::Token(token) => token.kind.to_string(),
    };

    if let Some(name) = cursor.node_name() {
        format!("({name}꞉ {kind})", name = name.as_ref().to_snake_case())
    } else {
        format!("({kind})")
    }
}

fn render_value(cursor: &mut CursorWithNames, source: &str) -> Result<String> {
    let value = match cursor.node() {
        Node::Rule(rule) if rule.children.is_empty() => "[]".to_string(),
        Node::Rule(_) => String::new(),
        Node::Token(_) => render_source_preview(cursor, source)?,
    };

    Ok(value)
}

fn render_comment(cursor: &mut CursorWithNames, source: &str) -> Result<String> {
    let range = cursor.text_range();
    let range_string = format!("{range:?}", range = range.utf8());

    let comment = match cursor.node() {
        Node::Token(_) => range_string,
        Node::Rule(_) if range.is_empty() => range_string,
        Node::Rule(_) => {
            let source_preview = render_source_preview(cursor, source)?;
            format!("{range_string} {source_preview}")
        }
    };

    Ok(comment)
}

fn render_source_preview(cursor: &mut CursorWithNames, source: &str) -> Result<String> {
    let range = cursor.text_range().utf8();
    let length = range.len();

    // Trim long values:
    let max_length = 50;
    let contents = source
        .bytes()
        .skip(range.start)
        .take(length.clamp(0, max_length))
        .collect();

    // Add terminator if trimmed:
    let mut contents = String::from_utf8(contents)?;
    if length > max_length {
        contents.push_str("...");
    }

    // Escape line breaks:
    let contents = contents
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n");

    // Surround by quotes for use in yaml:
    let contents = if contents.contains('"') {
        let contents = contents.replace('\'', "''");
        format!("'{contents}'")
    } else {
        let contents = contents.replace('"', "\\\"");
        format!("\"{contents}\"")
    };

    Ok(contents)
}
