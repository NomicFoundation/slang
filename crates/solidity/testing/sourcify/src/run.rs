use anyhow::{bail, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use slang_solidity::compilation::{CompilationInitializationError, CompilationUnit};
use slang_solidity::cst::{Cursor, NodeKind, NonterminalKind, TerminalKindExtensions, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};
use slang_solidity::utils::LanguageFacts;

use crate::command::TestOptions;
use crate::events::{Events, TestOutcome};
use crate::sourcify::{Contract, ContractArchive, Manifest};

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
            let mut test_outcome = run_parser_check(contract, &unit, events);

            if opts.check_infer_version && test_outcome == TestOutcome::Passed {
                test_outcome = run_version_inference_check(contract, &unit, events);
            }

            if opts.check_bindings && test_outcome == TestOutcome::Passed {
                test_outcome = run_bindings_check(contract, &unit, events);
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

                TestOutcome::Unresolved
            }
        }
    };

    events.inc_files_processed(sources_count);
    events.test(test_outcome);
}

fn run_parser_check(contract: &Contract, unit: &CompilationUnit, events: &Events) -> TestOutcome {
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

fn run_version_inference_check(
    contract: &Contract,
    unit: &CompilationUnit,
    events: &Events,
) -> TestOutcome {
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

fn run_bindings_check(
    contract: &Contract,
    compilation_unit: &CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let binding_graph = compilation_unit.binding_graph();

    let mut test_outcome = TestOutcome::Passed;
    for reference in binding_graph.all_references() {
        let ref_file = reference.get_file();

        if ref_file.is_built_ins() {
            // skip built-ins
            continue;
        }
        // We're not interested in the exact definition a reference resolves
        // to, so we lookup all of them and fail if we find none.
        if reference.definitions().is_empty() {
            let cursor = reference.get_cursor().to_owned();

            let source = contract.read_file(ref_file.get_path()).unwrap();

            let binding_error = BindingError::UnresolvedReference(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                ref_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Reference has no definitions\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            test_outcome = TestOutcome::Failed;
        }
    }

    // Check that all identifier nodes are bound to either a definition or a reference:
    for file in compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            if matches!(
                cursor.ancestors().next(),
                Some(ancestor)
                // Ignore identifiers in certain pragma contexts
                // `pragma experimental`: they are unbound feature names
                // `pragma abicoder`: they are unbound abi version names
                if ancestor.kind == NonterminalKind::ExperimentalFeature || ancestor.kind == NonterminalKind::AbicoderPragma
            ) {
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph.reference_at(&cursor).is_none()
            {
                let binding_error = BindingError::UnboundIdentifier(cursor.clone());

                let source = contract.read_file(file.id()).unwrap();
                let msg =
                    slang_solidity::diagnostic::render(&binding_error, file.id(), &source, true);
                events.bindings_error(format!(
                    "[{contract_name} {version}] Binding Error: No definition or reference\n{msg}",
                    contract_name = contract.name,
                    version = contract.version,
                ));

                test_outcome = TestOutcome::Failed;
            }
        }
    }

    test_outcome
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
}

impl Diagnostic for BindingError {
    fn text_range(&self) -> TextRange {
        let cursor = match self {
            Self::UnboundIdentifier(cursor) => cursor,
            Self::UnresolvedReference(cursor) => cursor,
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
        }
    }
}
