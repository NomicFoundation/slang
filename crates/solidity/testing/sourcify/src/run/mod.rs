use anyhow::{bail, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use slang_solidity::compilation::CompilationInitializationError;
use slang_solidity::cst::{Cursor, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};

use crate::command::{CheckBinderMode, TestOptions};
use crate::events::{Events, TestOutcome};
use crate::sourcify::{Contract, ContractArchive, Manifest};

mod binder_v1_check;
mod binder_v2_check;
mod compare_binders;
mod parser_check;
mod version_inference_check;

pub fn test_single_contract(
    manifest: &Manifest,
    contract_id: &str,
    opts: &TestOptions,
) -> Result<()> {
    let Some(contract) = manifest.fetch_contract(contract_id) else {
        bail!("Contract {contract_id} not found");
    };

    let mut events = Events::new(1, 0);

    events.start_archive(1);
    run_test(&contract, &events, opts);
    events.finish_archive();

    Ok(())
}

pub fn run_with_trace(archive: &ContractArchive, events: &Events, opts: &TestOptions) {
    for contract in archive.contracts() {
        events.trace(format!(
            "[{name} {version}] Starting contract",
            version = contract.version,
            name = contract.name
        ));
        run_test(&contract, events, opts);
        events.trace(format!(
            "[{name} {version}] Finished contract",
            version = contract.version,
            name = contract.name
        ));
    }
}

pub fn run_in_parallel(archive: &ContractArchive, events: &Events, opts: &TestOptions) {
    archive
        .contracts()
        .par_bridge()
        .panic_fuse()
        .for_each(|contract| run_test(&contract, events, opts));
}

fn run_test(contract: &Contract, events: &Events, opts: &TestOptions) {
    if uses_exotic_parser_bug(contract) {
        events.test(TestOutcome::Incompatible);
        return;
    }

    let sources_count = contract.sources_count();
    events.inc_files_count(sources_count);

    let test_outcome = match contract.create_compilation_unit() {
        Ok(unit) => {
            let mut test_outcome = parser_check::run(contract, &unit, events);

            if opts.check_infer_version && test_outcome == TestOutcome::Passed {
                test_outcome = version_inference_check::run(contract, &unit, events);
            }

            if test_outcome == TestOutcome::Passed {
                match opts.check_binder {
                    Some(CheckBinderMode::V1) => {
                        test_outcome = binder_v1_check::run(contract, &unit, events);
                    }
                    Some(CheckBinderMode::V2) => {
                        test_outcome = binder_v2_check::run(contract, unit, events);
                    }
                    Some(CheckBinderMode::Compare) => {
                        test_outcome = compare_binders::run(contract, unit, events);
                    }
                    _ => {}
                }
            }

            test_outcome
        }
        Err(e) => {
            if let Some(CompilationInitializationError::UnsupportedLanguageVersion(_)) =
                e.downcast_ref::<CompilationInitializationError>()
            {
                TestOutcome::Incompatible
            } else {
                events.trace(format!(
                    "Failed to compile contract {}: {e}\n{}",
                    contract.name,
                    e.backtrace()
                ));

                TestOutcome::Failed
            }
        }
    };

    events.inc_files_processed(sources_count);
    events.test(test_outcome);
}

fn uses_exotic_parser_bug(contract: &Contract) -> bool {
    static CONTRACTS_WITH_EXOTIC_PARSER_BUGS: &[&str] = &[
        // 0.4.24: // Accepts malformed `* /` in multi-line comments:
        // Fixed in 0.4.25: https://github.com/ethereum/solidity/pull/4937
        "0x79Bb6f4492D5CB13Fad8cA0ecfBccD9e2c26ac42",
        // 0.5.11: Double `indexed` keyword
        // Fixed in 0.8.18: https://github.com/ethereum/solidity/blob/develop/Changelog.md#0818-2023-02-01
        "0x9F4F8Cb4863D3467F03773cC4c172837106C21D8",
        // 0.5.16: Double `indexed` keyword
        // Fixed in 0.8.18: https://github.com/ethereum/solidity/blob/develop/Changelog.md#0818-2023-02-01
        "0xDe201dAec04ba73166d9917Fdf08e1728E270F06",
        // 0.4.19: Unclosed multi-line comment at EOF
        "0xf330AA697a1128B7A2D2204F6794afe0cAAF58FC",
    ];

    CONTRACTS_WITH_EXOTIC_PARSER_BUGS
        .iter()
        .any(|c| c == &contract.name)
}

enum BindingError {
    UnresolvedReference(Cursor),
    UnboundIdentifier(Cursor),
    MissingDefinition(Cursor),
    MissingReference(Cursor),
}

impl Diagnostic for BindingError {
    fn text_range(&self) -> TextRange {
        let cursor = match self {
            Self::UnboundIdentifier(cursor)
            | Self::UnresolvedReference(cursor)
            | Self::MissingDefinition(cursor)
            | Self::MissingReference(cursor) => cursor,
        };
        cursor.text_range()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match self {
            Self::UnresolvedReference(cursor) => {
                format!(
                    "Unresolved reference to `{symbol}`",
                    symbol = cursor.node().unparse()
                )
            }
            Self::UnboundIdentifier(cursor) => {
                format!(
                    "Missing identifier or definition for `{symbol}`",
                    symbol = cursor.node().unparse()
                )
            }
            Self::MissingDefinition(cursor) => {
                format!(
                    "Definition for `{symbol}` not found in new binder",
                    symbol = cursor.node().unparse()
                )
            }
            Self::MissingReference(cursor) => {
                format!(
                    "Reference for `{symbol}` not found in new binder",
                    symbol = cursor.node().unparse()
                )
            }
        }
    }
}
