use semver::Version;
use slang_solidity::compilation::CompilationUnit;

use super::print_errors;
use crate::events::{Events, SingleTestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(
    contract: &Contract,
    unit: &CompilationUnit,
    events: &Events,
) -> Option<SingleTestOutcome> {
    if contract.version < Version::new(0, 8, 0) {
        return None;
    }

    let mut test_outcome = SingleTestOutcome::Passed;

    for file in unit.files() {
        let source = contract.read_file(file.id()).unwrap();

        let v2_errors = solidity_v2_testing_utils::v1_comparison::parser::compare_with_v1_cursor(
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
            test_outcome = SingleTestOutcome::Failed;
        }
    }

    Some(test_outcome)
}
