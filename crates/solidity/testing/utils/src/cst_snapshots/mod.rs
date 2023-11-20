mod test_nodes;

use std::{cmp::max, fmt::Write};

use anyhow::Result;
use slang_solidity::{
    cst,
    cursor::Cursor,
    kinds::TokenKind,
    text_index::{TextRange, TextRangeExtensions},
};

use crate::{cst_snapshots::test_nodes::TestNode, node_extensions::NodeExtensions};

pub struct CstSnapshots;

impl CstSnapshots {
    pub fn render(source: &str, errors: &Vec<String>, cursor: Cursor) -> Result<String> {
        let mut w = String::new();

        write_source(&mut w, source)?;
        writeln!(&mut w)?;

        write_errors(&mut w, errors)?;
        writeln!(&mut w)?;

        write_tree(&mut w, cursor, source)?;

        return Ok(w);
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
    for (index, line, bytes, chars) in line_data.iter() {
        let range = offset..(offset + bytes);
        writeln!(
            w,
            "  {line_number: <2} │ {line}{padding} │ {range:?}",
            line_number = index + 1,
            padding = " ".repeat(source_width - chars),
        )?;

        offset = range.end + 1;
    }

    return Ok(());
}

fn write_errors<W: Write>(w: &mut W, errors: &Vec<String>) -> Result<()> {
    if errors.len() == 0 {
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

    return Ok(());
}

fn write_tree<W: Write>(w: &mut W, mut cursor: Cursor, source: &str) -> Result<()> {
    write!(w, "Tree:")?;
    writeln!(w)?;

    let significant_nodes_with_range = std::iter::from_fn(|| loop {
        let (depth, range) = (cursor.depth(), cursor.text_range());

        // Skip whitespace and trivia rules containing only those tokens
        match cursor.next() {
            Some(cst::Node::Rule(rule))
                if rule.is_trivia()
                    && rule.children.iter().all(|token| {
                        token
                            .as_token_with_kind(&[TokenKind::Whitespace, TokenKind::EndOfLine])
                            .is_some()
                    }) =>
            {
                continue
            }
            Some(cst::Node::Token(token)) if token.kind == TokenKind::Whitespace => continue,
            Some(cst::Node::Token(token)) if token.kind == TokenKind::EndOfLine => continue,
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
    node: &cst::Node,
    range: &TextRange,
    source: &str,
    indentation: usize,
) -> Result<()> {
    let range_string = format!("{range:?}", range = range.utf8());

    let (node_value, node_comment) = if range.is_empty() {
        let preview = match node {
            cst::Node::Rule(_) => " []",
            cst::Node::Token(_) => " \"\"",
        };

        (preview.to_owned(), range_string)
    } else {
        let preview = TestNode::render_source_preview(source, &range)?;

        if node.children().is_empty() {
            // "foo" # 1..2
            (format!(" {preview}"), range_string)
        } else {
            // # 1..2 "foo"
            ("".to_owned(), format!("{range_string} {preview}"))
        }
    };

    let name = match node {
        cst::Node::Rule(rule) => format!("{:?} (Rule)", rule.kind),
        cst::Node::Token(token)
            if matches!(
                token.kind,
                TokenKind::SingleLineComment | TokenKind::MultilineComment
            ) =>
        {
            format!("{:?} (Trivia)", token.kind)
        }
        cst::Node::Token(token) => format!("{:?} (Token)", token.kind),
    };

    writeln!(
        w,
        "{indentation}  - {name}:{node_value} # {node_comment}",
        indentation = " ".repeat(4 * indentation),
    )?;

    Ok(())
}
