use slang_solidity::backend::build_binder_output;
use slang_solidity::compilation::CompilationUnit;

use super::BindingError;
use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(
    contract: &Contract,
    compilation_unit: CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let data = build_binder_output(compilation_unit);
    let binder = data.binder;

    let binding_graph = data.compilation_unit.binding_graph();

    let mut outcome = TestOutcome::Passed;

    let mut definitions = 0;
    for definition in binding_graph.all_definitions() {
        let def_file = definition.get_file();
        if def_file.is_built_ins() {
            continue;
        }
        definitions += 1;

        if binder.find_definition_by_id(definition.id()).is_none() {
            // definition doesn't exist in new binder
            let cursor = definition.get_cursor().to_owned();
            let source = contract.read_file(def_file.get_path()).unwrap_or_default();
            let binding_error = BindingError::MissingDefinition(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                def_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Definition does not exist in new binder\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            outcome = TestOutcome::Failed;
        }
    }
    events.inc_definitions(definitions);

    let mut references = 0;
    let mut unresolved = 0;
    for reference in binding_graph.all_references() {
        let ref_file = reference.get_file();
        if ref_file.is_built_ins() {
            continue;
        }
        if reference.definitions().is_empty() {
            unresolved += 1;
        }
        references += 1;

        // Only check that the reference exists in the new binder
        if binder
            .find_reference_by_identifier_node_id(reference.id())
            .is_none()
        {
            // reference doesn't exist in new binder
            let cursor = reference.get_cursor().to_owned();
            let source = contract.read_file(ref_file.get_path()).unwrap_or_default();
            let binding_error = BindingError::MissingReference(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                ref_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Reference does not exist in new binder\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            outcome = TestOutcome::Failed;
        }
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    outcome
}
