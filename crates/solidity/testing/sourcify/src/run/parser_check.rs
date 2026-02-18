use semver::Version;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::diagnostic::Diagnostic;

use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut test_outcome = TestOutcome::Passed;

    for file in unit.files() {
        let v1_errors = file.errors();

        if !v1_errors.is_empty() {
            print_errors(contract, events, file.id(), v1_errors);
            test_outcome = TestOutcome::Failed;
            continue;
        }

        let source = contract.read_file(file.id()).unwrap();
        let v2_errors = slang_solidity_v2_parser::temp_sourcify::Comparator::compare_with_v1_output(
            contract.version.clone(),
            &source,
            file.create_tree_cursor(),
        );

        if !v2_errors.is_empty() {
            print_errors(contract, events, file.id(), &v2_errors);
            test_outcome = TestOutcome::Failed;
            continue;
        }

        if contract.version >= Version::new(0, 8, 0) && contract.version <= Version::new(0, 8, 33) {
            let v2_errors = slang_solidity_v2_parser::temp_testing::compare_with_v1_cursor(
                &source,
                &file.create_tree_cursor(),
            );
            if !v2_errors.is_empty() {
                print_errors(contract, events, file.id(), &v2_errors);
                test_outcome = TestOutcome::Failed;
                continue;
            }
        }
    }

    test_outcome
}

fn print_errors(
    contract: &Contract,
    events: &Events,
    file_id: &str,
    errors: &Vec<impl Diagnostic>,
) {
    let source = contract.read_file(file_id).unwrap();
    let source_name = contract
        .import_resolver
        .get_virtual_path(file_id)
        .unwrap_or(file_id.into());

    for error in errors {
        let msg = slang_solidity::diagnostic::render(error, &source_name, &source, true);
        events.parse_error(format!(
            "[{contract_name} {version}] Parse error\n{msg}",
            contract_name = contract.name,
            version = contract.version
        ));
    }
}
