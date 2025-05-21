use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{anyhow, Result};
use inflector::Inflector;
use slang_solidity::backend::passes::{
    p0_build_ast, p1_flatten_contracts, p2_collect_types, p3_annotate_types,
};
use slang_solidity::backend::types::{TypeId, TypeRegistry};
use slang_solidity::cst::{Cursor, Node, NodeId, TerminalKind};

#[test]
fn test_annotate_types_pass() -> Result<()> {
    let unit = super::build_compilation_unit()?;

    let data = p0_build_ast::run(&unit).map_err(|s| anyhow!(s))?;
    let data = p1_flatten_contracts::run(data);
    let data = p2_collect_types::run(data);
    let data = p3_annotate_types::run(data);

    assert_eq!(2, data.files.len());

    let mut w = String::new();
    render_types(&mut w, &data.types)?;

    for file in &unit.files() {
        render_tree(
            &mut w,
            file.id(),
            file.create_tree_cursor(),
            &data.annotations,
        )?;
    }
    println!("{w}");

    Ok(())
}

fn render_types(w: &mut String, types: &TypeRegistry) -> Result<()> {
    writeln!(w, "-------------------------------------------------------")?;
    writeln!(w, "Registered types:")?;
    for (type_id, type_) in types.iter_types() {
        writeln!(w, "{type_id:?} -> {type_:?}")?;
    }

    writeln!(w, "-------------------------------------------------------")?;
    writeln!(w, "Registered definitions:")?;
    for definition in types.iter_definitions() {
        writeln!(w, "{definition:#?}")?;
    }
    Ok(())
}

fn render_tree(
    w: &mut String,
    file_id: &str,
    mut cursor: Cursor,
    annotations: &HashMap<NodeId, TypeId>,
) -> Result<()> {
    writeln!(w, "Tree for {file_id}:")?;
    render_node(w, &mut cursor, 0, annotations)?;

    assert!(!cursor.go_to_next());
    Ok(())
}

fn render_node(
    w: &mut String,
    cursor: &mut Cursor,
    depth: usize,
    annotations: &HashMap<NodeId, TypeId>,
) -> Result<()> {
    if cursor.node().is_trivia()
        || cursor.node().is_terminal_with_kinds(&[
            TerminalKind::Semicolon,
            TerminalKind::OpenBrace,
            TerminalKind::OpenBracket,
            TerminalKind::OpenParen,
            TerminalKind::CloseBrace,
            TerminalKind::CloseBracket,
            TerminalKind::CloseParen,
            TerminalKind::Comma,
        ])
    {
        return Ok(());
    }
    let indentation = " ".repeat(4 * depth);
    let type_id = if let Some(type_id) = annotations.get(&cursor.node().id()) {
        format!("{type_id:?}")
    } else {
        String::new()
    };
    write!(w, "{type_id:15}{indentation}  - ")?;

    write!(w, "{key}", key = render_key(cursor))?;
    writeln!(w, ": {value}", value = render_value(cursor))?;

    for _ in cursor.node().children() {
        assert!(cursor.go_to_next());
        render_node(w, cursor, depth + 1, annotations)?;
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

fn render_value(cursor: &mut Cursor) -> String {
    if cursor.node().is_terminal() {
        cursor.node().unparse()
    } else {
        String::new()
    }
}
