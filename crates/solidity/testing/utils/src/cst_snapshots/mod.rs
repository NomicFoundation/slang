mod test_nodes;

use std::{self, cmp::max, fmt::Write};

use anyhow::Result;
use slang_solidity::syntax::parser::ParseOutput;

use crate::cst_snapshots::test_nodes::TestNode;

pub trait ParseOutputTestSnapshotExtensions {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String>;
}

impl ParseOutputTestSnapshotExtensions for ParseOutput {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String> {
        let mut w = String::new();

        write_source(&mut w, source)?;
        writeln!(&mut w)?;

        write_errors(&mut w, &self, source_id, source)?;
        writeln!(&mut w)?;

        write_tree(&mut w, &self, source)?;

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

fn write_errors<W: Write>(
    w: &mut W,
    output: &ParseOutput,
    source_id: &str,
    source: &str,
) -> Result<()> {
    let errors = output.errors();

    if errors.len() == 0 {
        writeln!(w, "Errors: []")?;
        return Ok(());
    }

    writeln!(w, "Errors: # {count} total", count = errors.len())?;

    for error in errors {
        writeln!(w, "  - >")?;
        let report = error.to_error_report(source_id, source, /* with_colour */ false);
        for line in report.lines() {
            writeln!(w, "    {line}")?;
        }
    }

    return Ok(());
}

fn write_tree<W: Write>(w: &mut W, output: &ParseOutput, source: &str) -> Result<()> {
    let root_node = if let Some(parse_tree) = output.parse_tree() {
        TestNode::from_cst(&parse_tree)
    } else {
        writeln!(w, "Tree: null")?;
        return Ok(());
    };

    writeln!(w, "Tree:")?;
    write_node(w, &root_node, source, 0)?;
    return Ok(());
}

fn write_node<W: Write>(
    w: &mut W,
    node: &TestNode,
    source: &str,
    indentation: usize,
) -> Result<()> {
    let (node_value, node_comment) = if let Some(range) = &node.range {
        let preview = node.render_preview(source, range)?;
        if node.children.is_empty() {
            (
                format!(" {preview}"),
                format!("{range:?}", range = range.start.utf8..range.end.utf8),
            )
        } else {
            (
                "".to_owned(),
                format!(
                    "{range:?} {preview}",
                    range = range.start.utf8..range.end.utf8
                ),
            )
        }
    } else {
        (" \"\"".to_owned(), "<empty>".to_owned())
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
