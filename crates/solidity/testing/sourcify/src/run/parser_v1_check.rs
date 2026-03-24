use slang_solidity::compilation::CompilationUnit;

use super::print_errors;
use crate::events::{Events, SingleTestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(
    contract: &Contract,
    unit: &CompilationUnit,
    events: &Events,
) -> SingleTestOutcome {
    let mut v1_outcome = SingleTestOutcome::Passed;

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
            v1_outcome = SingleTestOutcome::Failed;
        }
    }

    v1_outcome
}
