use semver::Version;
use slang_solidity::compilation::CompilationUnit;

use crate::events::{Events, TestOutcome, VersionOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut v1_outcome = VersionOutcome::Passed;
    let mut v2_outcome = if contract.version >= Version::new(0, 8, 0) {
        VersionOutcome::Passed
    } else {
        VersionOutcome::NotTested
    };

    for file in unit.files() {
        let v1_errors = file.errors();

        if !v1_errors.is_empty() {
            print_errors(
                contract,
                events,
                file.id(),
                v1_errors,
                slang_solidity::diagnostic::render,
            );
            v1_outcome = VersionOutcome::Failed;
        }

        // The V2 language only supports 0.8.0 and above
        if contract.version >= Version::new(0, 8, 0) {
            let source = contract.read_file(file.id()).unwrap();

            let v2_errors =
                solidity_v2_testing_utils::v1_comparison::parser::compare_with_v1_cursor(
                    &source,
                    &file.create_tree_cursor(),
                    &contract.version,
                );
            if !v2_errors.is_empty() {
                print_errors(
                    contract,
                    events,
                    file.id(),
                    &v2_errors,
                    solidity_v2_testing_utils::reporting::diagnostic::render,
                );
                v2_outcome = VersionOutcome::Failed;
            }
        }
    }

    TestOutcome::Tested {
        v1: v1_outcome,
        v2: v2_outcome,
    }
}

/// Given a vector of errors, and a function that can render them,
/// render them together and pass them as events
fn print_errors<T, F>(
    contract: &Contract,
    events: &Events,
    file_id: &str,
    errors: &Vec<T>,
    render_fn: F,
) where
    F: Fn(&T, &str, &str, bool) -> String,
{
    let source = contract.read_file(file_id).unwrap();
    let source_name = contract
        .import_resolver
        .get_virtual_path(file_id)
        .unwrap_or(file_id.into());

    for error in errors {
        let msg = render_fn(error, &source_name, &source, true);
        events.parse_error(format!(
            "[{contract_name} {version}] Parse error\n{msg}",
            contract_name = contract.name,
            version = contract.version
        ));
    }
}
