use slang_solidity::compilation::CompilationUnit;

use super::print_errors;
use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut outcome = TestOutcome::Passed;

    for file in unit.files() {
        let errors = file.errors();

        if !errors.is_empty() {
            print_errors(
                contract,
                events,
                file.id(),
                errors,
                slang_solidity::diagnostic::render,
            );
            outcome = TestOutcome::Failed;
        }
    }

    outcome
}
