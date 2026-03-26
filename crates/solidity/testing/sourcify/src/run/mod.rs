use anyhow::{bail, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use slang_solidity::compilation::CompilationInitializationError;
use slang_solidity::cst::{Cursor, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};

use crate::command::{CheckBinderMode, TestOptions};
use crate::events::{Events, SingleTestOutcome, TestCaseOutcome};
use crate::sourcify::{Contract, ContractArchive, Manifest};

mod binder_v1_check;
mod binder_v2_check;
mod compare_binders;
mod parser_v1_check;
mod parser_v2_check;
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
        events.test(TestCaseOutcome::Incompatible);
        return;
    }

    let sources_count = contract.sources_count();
    events.inc_files_count(sources_count);

    let test_outcome = match contract.create_compilation_unit() {
        Ok(unit) => {
            let mut v1_test_outcome = parser_v1_check::run(contract, &unit, events);

            if opts.check_infer_version && v1_test_outcome == SingleTestOutcome::Passed {
                v1_test_outcome = version_inference_check::run(contract, &unit, events);
            }

            if v1_test_outcome == SingleTestOutcome::Passed {
                match opts.check_binder {
                    Some(CheckBinderMode::V1) => {
                        v1_test_outcome = binder_v1_check::run(contract, &unit, events);
                    }
                    Some(CheckBinderMode::V2) => {
                        v1_test_outcome = binder_v2_check::run(contract, &unit, events);
                    }
                    Some(CheckBinderMode::Compare) => {
                        v1_test_outcome = compare_binders::run(contract, &unit, events);
                    }
                    _ => {}
                }
            }

            // TODO(v2): For now we only check V2 if V1 compiles, since V2 doesn't have a compilation unit.
            // Once it does the whole sourcify check should be independent one from another.
            let v2_test_outcome = parser_v2_check::run(contract, &unit, events);

            TestCaseOutcome::Tested {
                v1: v1_test_outcome,
                v2: v2_test_outcome,
            }
        }
        Err(e) => {
            if let Some(CompilationInitializationError::UnsupportedLanguageVersion(_)) =
                e.downcast_ref::<CompilationInitializationError>()
            {
                TestCaseOutcome::Incompatible
            } else {
                events.trace(format!(
                    "Failed to compile contract {}: {e}\n{}",
                    contract.name,
                    e.backtrace()
                ));

                TestCaseOutcome::Tested {
                    v1: SingleTestOutcome::Failed,
                    v2: None,
                }
            }
        }
    };

    events.inc_files_processed(sources_count);
    events.test(test_outcome);
}

fn uses_exotic_parser_bug(contract: &Contract) -> bool {
    static CONTRACTS_WITH_EXOTIC_PARSER_BUGS: &[&str] = &[
        // Accepts unterminated multi-line comments at EOF:
        // Fixed in 0.4.25: https://github.com/ethereum/solidity/pull/4937
        "0x50fEc5D840995B5fcca57D4e8F4c6695b2Dd411f",
        "0x5b8D776E4aECFebD8d03caD7d94f23424De1733A",
        "0x79Bb6f4492D5CB13Fad8cA0ecfBccD9e2c26ac42",
        "0x7d81c361d6aC60634117dD81Ab1b01b8Dc795A9D",
        "0xf330AA697a1128B7A2D2204F6794afe0cAAF58FC",
        // Double `indexed` keyword
        // Fixed in 0.8.18: https://github.com/ethereum/solidity/blob/develop/Changelog.md#0818-2023-02-01
        "0x5ED57b8f59f8d3bc805FC1087d8DE93C78A87305", // Polygon Mainnet (137)
        "0x9F4F8Cb4863D3467F03773cC4c172837106C21D8",
        "0xDe201dAec04ba73166d9917Fdf08e1728E270F06",
        // Invalid numeric literal `777777777_E18` (underscore before exponent)
        // https://docs.soliditylang.org/en/latest/grammar.html#syntax-rule-SolidityLexer.DecimalNumber
        "0xe483fdEa7403d28538bB2182a9f669709c297549", // Arbitrum One (42161)
        // Trailing comma in struct literal (fails from 0.4.12)
        // Unclear in which version this landed: https://github.com/argotorg/solidity/pull/2392
        "0x1D80F890f497b1672F9487862978032666179338",
        "0x66D58F0a2A44742688843cEB8C0Fa8d8567E3C54",
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
