mod chains;
mod command;
mod compilation_builder;
mod events;
mod import_resolver;
mod reporting;
mod results;
mod sourcify;

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use command::{Commands, ShowCombinedResultsCommand, TestOptions};
use events::{Events, TestOutcome};
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use results::{display_all_results, AllResults};
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{Cursor, NodeKind, NonterminalKind, TerminalKindExtensions, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};
use slang_solidity::utils::LanguageFacts;
use sourcify::{Contract, ContractArchive, Manifest};

fn main() -> Result<()> {
    let command::Cli { command } = command::Cli::parse();
    match command {
        Commands::Test(test_command) => run_test_command(test_command),
        Commands::ShowCombinedResults(results_command) => {
            run_show_combined_results_command(results_command)
        }
    }
}

// `cmd` is passed by value because it's consumed in the spawned thread,
// but for whatever reason clippy can't figure that out
#[allow(clippy::needless_pass_by_value)]
fn run_test_command(cmd: command::TestCommand) -> Result<()> {
    Terminal::step(format!(
        "Initialize {chain}/{network}",
        chain = cmd.chain.name(),
        network = cmd.chain.network_name()
    ));

    let manifest = Manifest::new(cmd.chain, &cmd.sharding_options)?;

    if let Some(contract) = &cmd.contract {
        return test_single_contract(&manifest, contract, &cmd.test_options);
    }

    let archive_count = manifest.archive_count();

    let (tx, rx) = std::sync::mpsc::channel::<ContractArchive>();

    // process_thread vs. fetching_thread - we don't want the thread to take ownership of repo
    // because then repo will be dropped as soon as the last archive is fetched (before processing
    // has finished), which will delete all the archive files
    let process_thread = std::thread::spawn(move || -> Events {
        let mut events = Events::new(archive_count, 0);
        for archive in rx {
            Terminal::step(archive.display_path());

            events.start_archive(archive.contract_count());
            if cmd.trace {
                run_with_trace(&archive, &events, &cmd.test_options);
            } else {
                run_in_parallel(&archive, &events, &cmd.test_options);
            }
            events.finish_archive();

            if !cmd.save {
                archive.clean();
            }
        }

        events
    });

    // Fetching the shards in this closure so that it takes ownership of the sender
    // The sender needs to be dropped so that process_thread can finish
    let fetcher = |t: std::sync::mpsc::Sender<ContractArchive>| {
        for archive in manifest.archives() {
            t.send(archive).unwrap();
        }
    };

    fetcher(tx);

    let events = process_thread.join().unwrap();

    if GitHub::is_running_in_ci() {
        let output_path = PathBuf::from("target").join("__SLANG_SOURCIFY_SHARD_RESULTS__.json");
        let results = events.to_results();
        let value = serde_json::to_string(&results)?;

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        output_path.write_string(value)?;
        println!("Wrote results to {output_path:?}");
    }

    let failure_count = events.failure_count();
    if failure_count > 0 {
        println!(
            "\nFound {failure_count} failure(s). Please check the logs above for more information.\n",
        );
    }

    Ok(())
}

fn test_single_contract(manifest: &Manifest, contract_id: &str, opts: &TestOptions) -> Result<()> {
    if let Some(contract) = manifest.get_contract(contract_id) {
        let mut events = Events::new(1, 0);

        events.start_archive(1);
        run_test(&contract, &events, opts);
        events.finish_archive();

        return Ok(());
    }

    bail!("Contract {contract_id} not found");
}

fn run_with_trace(archive: &ContractArchive, events: &Events, opts: &TestOptions) {
    for contract in archive.contracts() {
        events.trace(format!(
            "[{version}] Starting contract {name}",
            version = contract.version,
            name = contract.name
        ));
        run_test(&contract, events, opts);
        events.trace(format!(
            "[{version}] Finished contract {name}",
            version = contract.version,
            name = contract.name
        ));
    }
}

fn run_in_parallel(archive: &ContractArchive, events: &Events, opts: &TestOptions) {
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

    let mut test_outcome = TestOutcome::Passed;
    match contract.create_compilation_unit() {
        Ok(unit) => {
            if opts.check_parser {
                test_outcome = run_parser_check(contract, &unit, events).unwrap_or(TestOutcome::Failed);
            }

            if opts.check_infer_version && test_outcome == TestOutcome::Passed {
                test_outcome = run_version_inference_check(contract, &unit, events).unwrap_or(TestOutcome::Failed);
            }

            if opts.check_bindings && test_outcome == TestOutcome::Passed {
                test_outcome = run_bindings_check(contract, &unit, events).unwrap_or(TestOutcome::Failed);
            }
        }
        Err(e) => {
            events.trace(format!(
                "Failed to compile contract {}: {e}\n{}",
                contract.name,
                e.backtrace()
            ));
            test_outcome = TestOutcome::Unresolved;
        }
    }

    events.inc_files_processed(sources_count);
    events.test(test_outcome);
}

fn run_parser_check(contract: &Contract, unit: &CompilationUnit, events: &Events) -> Result<TestOutcome> {
    let mut test_outcome = TestOutcome::Passed;
    for file in unit.files() {
        if !file.errors().is_empty() {
            let source = contract.read_file(file.id())?;

            let source_name = contract
                .import_resolver
                .get_virtual_path(file.id())
                .unwrap_or(file.id().into());

            for error in file.errors() {
                let msg =
                    slang_solidity::diagnostic::render(error, &source_name, &source, true);
                events.parse_error(format!(
                    "[{version}] Parse error in contract {contract_name}\n{msg}",
                    contract_name = contract.name,
                    version = contract.version
                ));
            }
            test_outcome = TestOutcome::Failed;
        }
    }

    Ok(test_outcome)
}

fn run_version_inference_check(
    contract: &Contract,
    unit: &CompilationUnit,
    events: &Events,
) -> Result<TestOutcome> {
    let mut did_fail = false;
    for file in unit.files() {
        let source = contract.read_file(file.id())?;

        if !LanguageFacts::infer_language_versions(&source).any(|v| *v == contract.version) {
            let source_name = contract
                .import_resolver
                .get_real_name(file.id())
                .unwrap_or(file.id().into());
            events.version_error(format!(
                "[{version}] Could not infer correct version for {contract_name}:{source_name}",
                version = contract.version,
                contract_name = contract.name,
            ));
            did_fail = true;
        }
    }

    if did_fail {
        Ok(TestOutcome::Failed)
    } else {
        Ok(TestOutcome::Passed)
    }
}

fn run_bindings_check(
    contract: &Contract,
    compilation_unit: &CompilationUnit,
    events: &Events,
) -> Result<TestOutcome> {
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

            let source = cursor.node().unparse();
            let binding_error = BindingError::UnresolvedReference(cursor);
            let msg = slang_solidity::diagnostic::render(
                &binding_error,
                ref_file.get_path(),
                &source,
                true,
            );
            events.bindings_error(format!(
                "[{version}] Binding Error: Reference has no definitions\n{msg}",
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
                // ignore identifiers in `pragma experimental` directives, as they are unbound feature names:
                if ancestor.kind == NonterminalKind::ExperimentalFeature
            ) {
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph.reference_at(&cursor).is_none()
            {
                let binding_error = BindingError::UnboundIdentifier(cursor.clone());

                let source = contract.read_file(file.id())?;

                let msg =
                    slang_solidity::diagnostic::render(&binding_error, file.id(), &source, true);
                events.bindings_error(format!(
                    "[{version}] Binding Error: No definition or reference\n{msg}",
                    version = contract.version,
                ));

                test_outcome = TestOutcome::Failed;
            }
        }
    }

    Ok(test_outcome)
}

fn run_show_combined_results_command(command: ShowCombinedResultsCommand) -> Result<()> {
    let ShowCombinedResultsCommand { results_file } = command;

    let contents = results_file.read_to_string()?;
    let all_results: AllResults = serde_json::from_str(&contents)?;
    display_all_results(&all_results);
    Ok(())
}

fn uses_exotic_parser_bug(contract: &Contract) -> bool {
    static CONTRACTS_WITH_EXOTIC_PARSER_BUGS: &[&str] = &[
        // 0.4.24: // Accepts malformed `* /` in multi-line comments:
        // Fixed in 0.4.25: https://github.com/ethereum/solidity/pull/4937
        "0x79bb6f4492d5cb13fad8ca0ecfbccd9e2c26ac42",
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
