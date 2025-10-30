use std::collections::HashMap;
use std::rc::Rc;

use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::build_binder_output;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{Cursor, NodeId, SyntaxNode, TerminalKind};

use super::BindingError;
use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(
    contract: &Contract,
    compilation_unit: CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let data = build_binder_output(compilation_unit);

    let mut test_outcome = TestOutcome::Passed;
    let mut cursor_cache: HashMap<NodeId, (Cursor, String)> = HashMap::new();

    events.inc_definitions(data.binder.definitions().len());
    let mut references = 0;
    let mut unresolved = 0;
    for reference in data.binder.references().values() {
        references += 1;
        if !matches!(reference.resolution, Resolution::Unresolved) {
            continue;
        }
        unresolved += 1;
        test_outcome = TestOutcome::Failed;

        let (cursor, ref_file_id) = find_cursor_for_identifier(
            &mut cursor_cache,
            &data.compilation_unit,
            &reference.identifier,
        )
        .unwrap_or_else(|| {
            unreachable!(
                "cannot find Cursor pointing to {identifier:?}",
                identifier = reference.identifier
            )
        });

        let source = contract.read_file(&ref_file_id).unwrap_or_default();

        let binding_error = BindingError::UnresolvedReference(cursor);
        let msg = slang_solidity::diagnostic::render(&binding_error, &ref_file_id, &source, true);
        events.bindings_error(format!(
            "[{contract_name} {version}] Binding Error: Reference has no definitions\n{msg}",
            contract_name = contract.name,
            version = contract.version,
        ));
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    test_outcome
}

fn find_cursor_for_identifier(
    cursor_cache: &mut HashMap<NodeId, (Cursor, String)>,
    compilation_unit: &CompilationUnit,
    identifier: &Rc<SyntaxNode>,
) -> Option<(Cursor, String)> {
    if cursor_cache.is_empty() {
        for file in &compilation_unit.files() {
            let mut cursor = file.create_tree_cursor();
            while cursor.go_to_next_terminal_with_kinds(&[
                TerminalKind::Identifier,
                TerminalKind::YulIdentifier,
            ]) {
                cursor_cache.insert(cursor.node().id(), (cursor.clone(), file.id().to_string()));
            }
        }
    }
    cursor_cache.get(&identifier.id()).cloned()
}
