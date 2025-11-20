use slang_solidity::backend::binder::{Binder, Definition};
use slang_solidity::cst::{Cursor, TerminalKindExtensions};

// It takes all the identifiers listed from the cursor position, and returns all the
// definitions they refer to (if they are indeed references)
pub fn follow_all_references<'a>(binder: &'a Binder, root: &Cursor) -> Vec<&'a Definition> {
    let mut referenced_definitions = vec![];
    let mut cursor = root.spawn();
    while cursor.go_to_next_terminal() {
        let node = cursor.node();
        if node.is_nonterminal() && !node.as_terminal().unwrap().kind.is_identifier() {
            continue;
        }

        if let Some(definition) = binder.navigate_to(node.id()) {
            referenced_definitions.push(definition);
        }
    }
    referenced_definitions
}
