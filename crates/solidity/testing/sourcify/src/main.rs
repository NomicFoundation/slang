mod chains;
mod command;
mod compilation_builder;
mod events;
mod metadata;
mod reporting;
mod results;
mod sourcify;

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use command::{Commands, ShowCombinedResultsCommand};
use events::{Events, TestOutcome};
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use results::{display_all_results, AllResults};
use semver::Version;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{Cursor, NodeKind, NonterminalKind, TerminalKindExtensions, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};
use sourcify::{Contract, ContractArchive, Repository};

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

    if let Some(contract) = &cmd.contract {
        return test_contract(&cmd, contract);
    }

    let repo = Repository::new(cmd.chain, false)?;
    let shards = repo.fetch_shards(&cmd.sharding_options)?;

    let chain = cmd.chain;
    let shard_count = shards.len();

    let (tx, rx) = std::sync::mpsc::channel::<ContractArchive>();

    // process_thread vs. fetching_thread - we don't want the thread to take ownership of repo
    // because then repo will be dropped as soon as the last archive is fetched (before processing
    // has finished), which will delete all the archive files
    let process_thread = std::thread::spawn(move || -> Events {
        let mut events = Events::new(shard_count, 0);
        for archive in rx {
            Terminal::step(format!(
                "Testing shard {}/{:#04x}",
                archive.match_type.dir_name(),
                archive.id,
            ));
            events.start_directory(archive.contract_count());
            if cmd.trace {
                run_with_trace(&archive, &events, cmd.check_bindings);
            } else {
                run_in_parallel(&archive, &events, cmd.check_bindings);
            }
            events.finish_directory();
        }

        events
    });

    // Fetching the shards in this closure so that it takes ownership of the sender
    // The sender needs to be dropped so that process_thread can finish
    let fetcher = |t: std::sync::mpsc::Sender<ContractArchive>| {
        for shard in shards {
            match repo.fetch_archive(&shard, chain) {
                Ok(archive) => {
                    t.send(archive).unwrap();
                }
                Err(e) => eprintln!("Failed to create archive: {e}"),
            }
        }
    };

    fetcher(tx);

    let events = process_thread.join().unwrap();

    if GitHub::is_running_in_ci() {
        let output_path = PathBuf::from("target").join("__SLANG_SANCTUARY_SHARD_RESULTS__.json");
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

fn test_contract(cmd: &command::TestCommand, contract_id: &str) -> Result<()> {
    let repo = Repository::new(cmd.chain, cmd.save)?;
    let shards = repo.fetch_shards(&cmd.sharding_options)?;

    let contract_shard_id = u16::from_str_radix(contract_id.get(2..4).unwrap(), 16)?;

    let contract_shards = shards.iter().filter(|shard| shard.id == contract_shard_id);

    for contract_shard in contract_shards {
        let archive = repo.fetch_archive(contract_shard, cmd.chain)?;
        if let Ok(contract) = archive.get_contract(contract_id) {
            let mut events = Events::new(1, 0);
            events.start_directory(1);

            run_test(&contract, &events, false);

            events.finish_directory();

            return Ok(());
        }
    }

    bail!("Contract {contract_id} not found");
}

fn run_with_trace(archive: &ContractArchive, events: &Events, check_bindings: bool) {
    for contract in archive.contracts() {
        events.trace(format!(
            "[{version}] Starting contract {name}",
            version = contract.version(),
            name = contract.name
        ));
        run_test(&contract, events, check_bindings);
        events.trace(format!(
            "[{version}] Finished contract {name}",
            version = contract.version(),
            name = contract.name
        ));
    }
}

fn run_in_parallel(archive: &ContractArchive, events: &Events, check_bindings: bool) {
    archive
        .contracts()
        .par_bridge()
        .panic_fuse()
        .for_each(|contract| run_test(&contract, events, check_bindings));
}

fn run_test(contract: &Contract, events: &Events, check_bindings: bool) {
    if uses_exotic_parser_bug(contract) {
        events.test(TestOutcome::Incompatible);
        return;
    }

    let sources_count = contract.sources_count();
    events.inc_files_count(sources_count);

    let mut test_outcome = TestOutcome::Passed;
    match contract.create_compilation_unit() {
        Ok(unit) => {
            let mut source_buf = String::new();
            for file in unit.files() {
                if !file.errors().is_empty() {
                    source_buf.clear();
                    let _ = contract.read_file(file.id(), &mut source_buf);

                    let source_name = contract
                        .metadata
                        .get_virtual_path(file.id())
                        .unwrap_or(file.id().into());

                    for error in file.errors() {
                        let msg = slang_solidity::diagnostic::render(
                            error,
                            &source_name,
                            &source_buf,
                            true,
                        );
                        events.parse_error(format!(
                            "[{version}] Parse error in contract {contract_name}\n{msg}",
                            contract_name = contract.name,
                            version = contract.version()
                        ));
                    }
                    test_outcome = TestOutcome::Failed;
                }
            }

            let should_check_bindings = check_bindings && test_outcome == TestOutcome::Passed;
            if should_check_bindings
                && run_bindings_check(&unit, events, &contract.version()).is_err()
            {
                test_outcome = TestOutcome::Failed;
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

fn run_bindings_check(
    compilation_unit: &CompilationUnit,
    events: &Events,
    version: &Version,
) -> Result<(), BindingError> {
    let binding_graph = compilation_unit.binding_graph();

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

            let binding_error = BindingError::UnresolvedReference(cursor);
            events.bindings_error(format!(
                "[{version}] Binding Error: Reference has no definitions"
            ));

            return Err(binding_error);
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
                events.bindings_error(format!(
                    "[{version}] Binding Error: No definition or reference"
                ));

                return Err(binding_error);
            }
        }
    }

    Ok(())
}

fn run_show_combined_results_command(command: ShowCombinedResultsCommand) -> Result<()> {
    let ShowCombinedResultsCommand { results_file } = command;

    let contents = results_file.read_to_string()?;
    let all_results: AllResults = serde_json::from_str(&contents)?;
    display_all_results(&all_results);
    Ok(())
}
