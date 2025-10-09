use slang_solidity::compilation::CompilationUnit;

use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut test_outcome = TestOutcome::Passed;
    for file in unit.files() {
        if !file.errors().is_empty() {
            let source = contract.read_file(file.id()).unwrap();
            let source_name = contract
                .import_resolver
                .get_virtual_path(file.id())
                .unwrap_or(file.id().into());

            for error in file.errors() {
                let msg = slang_solidity::diagnostic::render(error, &source_name, &source, true);
                events.parse_error(format!(
                    "[{contract_name} {version}] Parse error\n{msg}",
                    contract_name = contract.name,
                    version = contract.version
                ));
            }

            test_outcome = TestOutcome::Failed;
        }
    }

    test_outcome
}
