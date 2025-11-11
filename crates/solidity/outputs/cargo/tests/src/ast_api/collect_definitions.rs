use slang_solidity::backend::binder::{Binder, Definition};
use slang_solidity::cst::{Cursor, TerminalKindExtensions};

pub(crate) fn collect_definitions<'a>(binder: &'a Binder, root: &Cursor) -> Vec<&'a Definition> {
    let mut cursor = root.spawn();
    let mut definitions = Vec::new();
    while cursor.go_to_next_terminal() {
        let node = cursor.node();
        if node.is_nonterminal() || !node.as_terminal().unwrap().kind.is_identifier() {
            continue;
        }
        if let Some(definition) = binder.find_definition_by_identifier_node_id(node.id()) {
            definitions.push(definition);
        }
    }
    definitions
}
