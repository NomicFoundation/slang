use slang_solidity::compilation::CompilationUnit;

use super::print_errors;
use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut v1_outcome = TestOutcome::Passed;

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
            v1_outcome = TestOutcome::Failed;
        }
    }

    v1_outcome
}
