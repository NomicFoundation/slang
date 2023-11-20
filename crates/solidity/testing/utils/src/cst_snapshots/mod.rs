mod test_nodes;

use std::{self, cmp::max, fmt::Write};

use anyhow::Result;
use slang_solidity::{cursor::Cursor, text_index::TextRangeExtensions};

use crate::cst_snapshots::test_nodes::{TestNode, TestNodeKind};

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

fn write_tree<W: Write>(w: &mut W, cursor: Cursor, source: &str) -> Result<()> {
    write!(w, "Tree:")?;
    writeln!(w)?;

    let tree = TestNode::from_cst(cursor);
    write_node(w, &tree, source, 0)?;

    return Ok(());
}

fn write_node<W: Write>(
    w: &mut W,
    node: &TestNode,
    source: &str,
    indentation: usize,
) -> Result<()> {
    let range_string = format!("{range:?}", range = node.range.utf8());

    let (node_value, node_comment) = if node.range.is_empty() {
        let preview = match node.kind {
            TestNodeKind::Rule(_) if !node.children.is_empty() => "",
            TestNodeKind::Rule(_) => " []",
            TestNodeKind::Token(_) | TestNodeKind::Trivia(_) => " \"\"",
        };
        (preview.to_owned(), range_string)
    } else {
        let preview = node.render_preview(source, &node.range)?;
        if node.children.is_empty() {
            (
                format!(" {preview}"),
                format!("{range:?}", range = node.range.utf8()),
            )
        } else {
            (
                "".to_owned(),
                format!("{range:?} {preview}", range = node.range.utf8()),
            )
        }
    };

    writeln!(
        w,
        "{indentation}  - {kind}:{node_value} # {node_comment}",
        indentation = " ".repeat(4 * indentation),
        kind = node.kind,
    )?;

    for child in &node.children {
        write_node(w, child, source, indentation + 1)?;
    }

    return Ok(());
}
