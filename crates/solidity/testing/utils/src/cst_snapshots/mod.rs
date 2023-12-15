use std::cmp::max;
use std::fmt::Write;

use anyhow::Result;
use slang_solidity::cst;
use slang_solidity::cursor::CursorWithNames;
use slang_solidity::kinds::TokenKind;
use slang_solidity::text_index::{TextRange, TextRangeExtensions};

use crate::node_extensions::NodeExtensions;

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
    write!(w, "Tree:")?;
    writeln!(w)?;

    let significant_nodes_with_range = std::iter::from_fn(|| loop {
        let (depth, range) = (cursor.depth(), cursor.text_range());

        // Skip whitespace and trivia rules containing only those tokens
        match cursor.next() {
            Some(cst::NamedNode {
                name: _,
                node: cst::Node::Rule(rule),
            }) if rule.is_trivia()
                && rule.children.iter().all(|named| {
                    named
                        .node
                        .as_token()
                        .map_or(false, |t| is_whitespace(t.kind))
                }) =>
            {
                continue
            }
            Some(cst::NamedNode {
                name: _,
                node: cst::Node::Token(token),
            }) if is_whitespace(token.kind) => continue,
            next => break next.map(|item| (item, depth, range)),
        }
    });

    for (node, depth, range) in significant_nodes_with_range {
        write_node(w, &node, &range, source, depth)?;
    }

    Ok(())
}

fn write_node<W: Write>(
    w: &mut W,
    cst::NamedNode { name, node }: &cst::NamedNode,
    range: &TextRange,
    source: &str,
    indentation: usize,
) -> Result<()> {
    let range_string = format!("{range:?}", range = range.utf8());

    let (node_value, node_comment) = if range.is_empty() {
        let preview = match node {
            cst::Node::Rule(_) if !node.children().is_empty() => "",
            cst::Node::Rule(_) => " []",
            cst::Node::Token(_) => " \"\"",
        };

        (preview.to_owned(), range_string)
    } else {
        let preview = render_source_preview(source, range)?;

        if node.children().is_empty() {
            // "foo" # 1..2
            (format!(" {preview}"), range_string)
        } else {
            // # 1..2 "foo"
            (String::new(), format!("{range_string} {preview}"))
        }
    };

    let field_name = if name.is_empty() {
        String::new()
    } else {
        format!("({name}) ")
    };

    let name = match node {
        cst::Node::Rule(rule) => format!("{field_name}{}", rule.kind.as_ref()),
        cst::Node::Token(token) => format!("{field_name}{}", token.kind.as_ref()),
    };

    writeln!(
        w,
        "{indentation}  - {name}:{node_value} # {node_comment}",
        indentation = " ".repeat(4 * indentation),
    )?;

    Ok(())
}

pub fn render_source_preview(source: &str, range: &TextRange) -> Result<String> {
    let max_length = 50;
    let length = range.end.utf8 - range.start.utf8;

    // Trim long values:
    let contents = source
        .bytes()
        .skip(range.start.utf8)
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
    let contents = {
        if contents.contains('"') {
            let contents = contents.replace('\'', "''");
            format!("'{contents}'")
        } else {
            let contents = contents.replace('"', "\\\"");
            format!("\"{contents}\"")
        }
    };

    Ok(contents)
}

fn is_whitespace(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Whitespace | TokenKind::EndOfLine)
}
