use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::events::{Events, TestOutcome};
use crate::sourcify::Contract;

pub(super) fn run(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
    let mut did_fail = false;
    for file in unit.files() {
        let source = contract.read_file(file.id()).unwrap();
        if !LanguageFacts::infer_language_versions(&source).any(|v| *v == contract.version) {
            let source_name = contract
                .import_resolver
                .get_source_id(file.id())
                .unwrap_or(file.id().into());
            events.version_error(format!(
                "[{contract_name} {version}] Could not infer correct version in file {source_name}",
                version = contract.version,
                contract_name = contract.name,
            ));
            did_fail = true;
        }
    }

    if did_fail {
        TestOutcome::Failed
    } else {
        TestOutcome::Passed
    }
}
