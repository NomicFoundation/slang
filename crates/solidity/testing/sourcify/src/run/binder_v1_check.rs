use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

use super::BindingError;
use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(
    contract: &Contract,
    compilation_unit: &CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let binding_graph = compilation_unit.binding_graph();

    let mut test_outcome = TestOutcome::Passed;

    let mut definitions = 0;
    for definition in binding_graph.all_definitions() {
        if definition.get_file().is_built_ins() {
            continue;
        }
        definitions += 1;
    }
    events.inc_definitions(definitions);

    let mut references = 0;
    let mut unresolved = 0;
    for reference in binding_graph.all_references() {
        let ref_file = reference.get_file();

        if ref_file.is_built_ins() {
            // skip built-ins
            continue;
        }

        references += 1;

        // We're not interested in the exact definition a reference resolves
        // to, so we lookup all of them and fail if we find none.
        if reference.definitions().is_empty() {
            unresolved += 1;
            let cursor = reference.get_cursor().to_owned();

            let source = contract.read_file(ref_file.get_path()).unwrap();

            let binding_error = BindingError::UnresolvedReference(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                ref_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Reference has no definitions\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            test_outcome = TestOutcome::Failed;
        }
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    // Check that all identifier nodes are bound to either a definition or a reference:
    for file in compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph.reference_at(&cursor).is_none()
            {
                let binding_error = BindingError::UnboundIdentifier(cursor.clone());

                let source = contract.read_file(file.id()).unwrap();
                let msg =
                    slang_solidity::diagnostic::render(&binding_error, file.id(), &source, true);
                events.bindings_error(format!(
                    "[{contract_name} {version}] Binding Error: No definition or reference\n{msg}",
                    contract_name = contract.name,
                    version = contract.version,
                ));

                test_outcome = TestOutcome::Failed;
            }
        }
    }

    test_outcome
}
