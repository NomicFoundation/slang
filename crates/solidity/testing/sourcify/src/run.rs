use std::rc::Rc;

use anyhow::{bail, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::passes;
use slang_solidity::compilation::{CompilationInitializationError, CompilationUnit};
use slang_solidity::cst::{
    Cursor, NodeKind, TerminalKind, TerminalKindExtensions, TerminalNode, TextRange,
};
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

            if test_outcome == TestOutcome::Passed {
                if opts.check_bindings {
                    test_outcome = run_bindings_check(contract, &unit, events);
                } else if opts.check_old_binder {
                    test_outcome = run_old_binder_check(contract, &unit, events);
                } else if opts.check_new_binder {
                    test_outcome = run_new_binder_check(contract, unit, events);
                } else if opts.compare_binders {
                    test_outcome = run_compare_binders(contract, unit, events);
                    assert!(test_outcome != TestOutcome::Failed, "fail-fast");
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

            test_outcome = TestOutcome::Unresolved;
        }
    }

    // Check that all identifier nodes are bound to either a definition or a reference:
    for file in compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
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

                test_outcome = TestOutcome::Unresolved;
            }
        }
    }

    test_outcome
}

fn run_old_binder_check(
    _contract: &Contract,
    compilation_unit: &CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let binding_graph = compilation_unit.binding_graph();
    let mut definitions = 0;
    for definition in binding_graph.all_definitions() {
        if definition.get_file().is_built_ins() {
            continue;
        }
        definitions += 1;
    }
    events.inc_definitions(definitions);
    let mut references = 0;
    let mut unresolved = 0;
    for reference in binding_graph.all_references() {
        if reference.get_file().is_built_ins() {
            // skip built-ins
            continue;
        }
        if reference.definitions().is_empty() {
            unresolved += 1;
        }
        references += 1;
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    TestOutcome::Passed
}

fn run_new_binder_check(
    contract: &Contract,
    compilation_unit: CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_type_definitions::run(data);
    let data = passes::p4_resolve_references::run(data);

    let mut test_outcome = TestOutcome::Passed;

    events.inc_definitions(data.binder.definitions().len());
    let mut references = 0;
    let mut unresolved = 0;
    for reference in data.binder.references().values() {
        references += 1;
        if !matches!(reference.resolution, Resolution::Unresolved) {
            continue;
        }
        unresolved += 1;
        test_outcome = TestOutcome::Unresolved;

        let Some((cursor, ref_file_path)) =
            find_cursor_for_identifier(&data.compilation_unit, &reference.identifier)
        else {
            events.bindings_error(format!(
                "[{contract_name} {version}] ERROR: cannot find Cursor pointing to {identifier:?}",
                identifier = reference.identifier,
                contract_name = contract.name,
                version = contract.version,
            ));
            panic!("this shouldn't happen");
        };

        let source = contract.read_file(&ref_file_path).unwrap_or_default();

        let binding_error = BindingError::UnresolvedReference(cursor);
        let msg = slang_solidity::diagnostic::render(&binding_error, &ref_file_path, &source, true);
        events.bindings_error(format!(
            "[{contract_name} {version}] Binding Error: Reference has no definitions\n{msg}",
            contract_name = contract.name,
            version = contract.version,
        ));
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    test_outcome
}

fn find_cursor_for_identifier(
    compilation_unit: &CompilationUnit,
    identifier: &Rc<TerminalNode>,
) -> Option<(Cursor, String)> {
    let node_id = identifier.id();
    for file in &compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next_terminal_with_kinds(&[
            TerminalKind::Identifier,
            TerminalKind::YulIdentifier,
        ]) {
            if cursor.node().id() == node_id {
                return Some((cursor, file.id().to_string()));
            }
        }
    }
    None
}

fn run_compare_binders(
    contract: &Contract,
    compilation_unit: CompilationUnit,
    events: &Events,
) -> TestOutcome {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_type_definitions::run(data);
    let data = passes::p4_resolve_references::run(data);
    let binder = data.binder;

    let binding_graph = data.compilation_unit.binding_graph();

    let mut outcome = TestOutcome::Passed;

    let mut definitions = 0;
    for definition in binding_graph.all_definitions() {
        let def_file = definition.get_file();
        if def_file.is_built_ins() {
            continue;
        }
        definitions += 1;

        if binder.find_definition_by_id(definition.id()).is_none() {
            // definition doesn't exist in new binder
            let cursor = definition.get_cursor().to_owned();
            let source = contract.read_file(def_file.get_path()).unwrap_or_default();
            let binding_error = BindingError::MissingDefinition(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                def_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Definition does not exist in new binder\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            outcome = TestOutcome::Failed;
        }
    }
    events.inc_definitions(definitions);

    let mut references = 0;
    let mut unresolved = 0;
    for reference in binding_graph.all_references() {
        let ref_file = reference.get_file();
        if ref_file.is_built_ins() {
            continue;
        }
        if reference.definitions().is_empty() {
            unresolved += 1;
        }
        references += 1;

        if binder
            .find_reference_by_identifier_node_id(reference.id())
            .is_none()
        {
            // reference doesn't exist in new binder
            let cursor = reference.get_cursor().to_owned();
            let source = contract.read_file(ref_file.get_path()).unwrap_or_default();
            let binding_error = BindingError::MissingReference(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                ref_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{contract_name} {version}] Binding Error: Reference does not exist in new binder\n{msg}",
                contract_name = contract.name,
                version = contract.version,
            ));

            outcome = TestOutcome::Failed;
        }
    }
    events.inc_references(references);
    events.inc_unresolved_references(unresolved);

    outcome
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
