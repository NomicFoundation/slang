//! `run-corpus`: compiles every contract of a corpus snapshot with Slang v2 and
//! classifies what it reports; `report`: the same census from saved results.

mod artifacts;
mod expected;
mod outcome;
mod report;
mod unit;

use std::cell::RefCell;
use std::io::{BufRead, BufWriter, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use anyhow::{Context, Result};
use artifacts::Layout;
use expected::ExpectedFailures;
use infra_utils::terminal::Terminal;
use outcome::{Check, Outcome, classify};
use rayon::prelude::*;
use report::Summary;
use slang_solidity_v2::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::collections::SortedMap;

use crate::command::{ReportCommand, ReportOptions, RunCorpusCommand};
use crate::corpus::{self, Corpus, CorpusContract};

/// The in-repo corpus of hand-written test cases, run by unit tests and the PR workflow.
#[cfg(test)]
pub fn testdata_corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata/corpus")
}

/// The checks against the record's solc artifacts, each skipped on its own when the
/// artifacts or the target contract are missing. A skipped layout check skips its
/// types check with it.
const ARTIFACT_CHECKS: [Check; 2] = [Check::StorageLayout, Check::TransientStorageLayout];

fn default_expected_failures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("expected-failures.toml")
}

pub fn run(cmd: &RunCorpusCommand) -> Result<()> {
    anyhow::ensure!(
        cmd.shard_index < cmd.shard_count,
        "shard-index must be less than shard-count"
    );
    let expected = load_expected(&cmd.report_options)?;

    let corpus = Corpus::open(&cmd.corpus)?;
    let paths: Vec<&PathBuf> = corpus.shard(cmd.shard_count, cmd.shard_index).collect();
    Terminal::step(format!(
        "Run corpus {corpus:?} ({description}): {count} contracts, shard {index}/{shards}",
        corpus = cmd.corpus,
        description = corpus.manifest.description,
        count = paths.len(),
        index = cmd.shard_index,
        shards = cmd.shard_count,
    ));

    if let Some(jobs) = cmd.jobs {
        rayon::ThreadPoolBuilder::new()
            .num_threads(jobs)
            .build_global()?;
    }
    install_panic_hook();

    let results = cmd
        .out
        .as_ref()
        .map(|path| -> Result<_> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            Ok(Mutex::new(BufWriter::new(std::fs::File::create(path)?)))
        })
        .transpose()?;
    let summary = Mutex::new(Summary::default());
    let done = AtomicUsize::new(0);
    let total = paths.len();

    paths.par_iter().try_for_each(|path| -> Result<()> {
        // A corpus is a curated artifact: an unreadable contract means the snapshot
        // itself is broken, so this fails the run instead of counting as a skip.
        let outcome = check(&corpus::read_contract(path)?, path, cmd.print_diagnostics);
        if let Some(results) = &results {
            let mut results = results.lock().expect("results lock");
            serde_json::to_writer(&mut *results, &outcome)?;
            results.write_all(b"\n")?;
        }
        summary.lock().expect("summary lock").add(&outcome);
        let n = done.fetch_add(1, Ordering::Relaxed) + 1;
        if n.is_multiple_of(1000) || n == total {
            eprintln!("{n}/{total}");
        }
        Ok(())
    })?;
    if let Some(results) = results {
        results.into_inner().expect("results lock").flush()?;
    }

    finish(
        &summary.into_inner().expect("summary lock"),
        &expected,
        &cmd.report_options,
    )
}

fn load_expected(options: &ReportOptions) -> Result<ExpectedFailures> {
    ExpectedFailures::load(
        options
            .expected_failures
            .as_deref()
            .unwrap_or(&default_expected_failures()),
    )
}

pub fn report(cmd: &ReportCommand) -> Result<()> {
    let expected = load_expected(&cmd.report_options)?;
    let mut summary = Summary::default();
    for path in &cmd.results {
        let file = std::fs::File::open(path).with_context(|| format!("Could not read {path:?}"))?;
        for line in std::io::BufReader::new(file).lines() {
            let outcome: Outcome = serde_json::from_str(&line?)
                .with_context(|| format!("Malformed outcome in {path:?}"))?;
            summary.add(&outcome);
        }
    }
    finish(&summary, &expected, &cmd.report_options)
}

fn finish(summary: &Summary, expected: &ExpectedFailures, options: &ReportOptions) -> Result<()> {
    let gate = summary.gate(expected, options.stale_check);
    let markdown = format!("{}\n{}", summary.markdown(expected), gate.markdown());
    println!("{markdown}");
    if let Some(path) = &options.report {
        std::fs::write(path, &markdown).with_context(|| format!("Could not write {path:?}"))?;
    }
    match gate.verdict() {
        Ok(()) => Ok(()),
        Err(problems) if options.report_only => {
            println!("Report only, not failing: {problems}");
            Ok(())
        }
        Err(problems) => anyhow::bail!("{problems}"),
    }
}

fn check(record: &CorpusContract, path: &Path, print_diagnostics: bool) -> Outcome {
    let id = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut outcome = Outcome {
        id,
        version: record.version.clone(),
        evm_target: None,
        ms: 0,
        skipped: None,
        panic: None,
        failures: Vec::new(),
        warnings: 0,
        skipped_checks: SortedMap::new(),
    };

    let (version, target) = match unit::language_version(record)
        .and_then(|version| unit::evm_target(record, version).map(|target| (version, target)))
    {
        Ok(config) => config,
        Err(skip) => {
            outcome.skipped = Some(skip.to_string());
            return outcome;
        }
    };
    outcome.evm_target = Some(target.to_string());

    let started = Instant::now();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let unit = unit::create(record, version, target);
        if print_diagnostics {
            print_errors(record, &outcome.id, unit.diagnostics().iter());
        }
        let (mut failures, warnings) = classify(
            unit.diagnostics()
                .iter()
                .map(|diagnostic| diagnostic.kind()),
        );
        let mut skipped_checks = SortedMap::new();
        let mut skip_all = |reason: &str| {
            for check in ARTIFACT_CHECKS {
                skipped_checks.insert(check.to_string(), reason.to_owned());
            }
        };
        match &record.artifacts {
            Some(artifacts) => match artifacts::target_definition(&unit, record) {
                Ok(target) => {
                    for layout in [Layout::Storage, Layout::Transient] {
                        match artifacts::check_storage_layout(&target, artifacts, layout) {
                            Ok(check_failures) => failures.extend(check_failures),
                            Err(reason) => {
                                skipped_checks.insert(layout.checks().0.to_string(), reason);
                            }
                        }
                    }
                }
                Err(reason) => skip_all(&reason),
            },
            None => skip_all("no artifacts"),
        }
        (failures, warnings, skipped_checks)
    }));
    outcome.ms = started.elapsed().as_millis();
    match result {
        Ok((failures, warnings, skipped_checks)) => {
            outcome.failures = failures;
            outcome.warnings = warnings;
            outcome.skipped_checks = skipped_checks;
        }
        Err(_) => outcome.panic = Some(take_panic_message()),
    }
    outcome
}

fn print_errors<'a>(
    record: &CorpusContract,
    id: &str,
    diagnostics: impl Iterator<Item = &'a Diagnostic>,
) {
    for diagnostic in diagnostics {
        if diagnostic.kind().severity() != DiagnosticSeverity::Error {
            continue;
        }
        let file = diagnostic.file_id().as_str();
        let line = record
            .sources
            .get(file)
            .map(|source| {
                source[..diagnostic.text_range().start.min(source.len())]
                    .matches('\n')
                    .count()
                    + 1
            })
            .unwrap_or_default();
        println!(
            "{id} {file}:{line} {code}: {message}",
            code = diagnostic.kind().code(),
            message = diagnostic.kind().message()
        );
    }
}

thread_local! {
    static LAST_PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Records each panic's message and location for the outcome instead of printing it;
/// with tens of thousands of contracts the default hook would flood stderr.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = info
            .payload()
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| {
                info.payload()
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
            })
            .unwrap_or_else(|| "non-string panic".to_owned());
        let location = info
            .location()
            .map(|location| format!(" at {}:{}", location.file(), location.line()))
            .unwrap_or_default();
        LAST_PANIC.with(|last| *last.borrow_mut() = Some(format!("{message}{location}")));
    }));
}

fn take_panic_message() -> String {
    LAST_PANIC
        .with(|last| last.borrow_mut().take())
        .unwrap_or_else(|| "panic without message".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_gate_is_written_into_the_report() {
        let record: CorpusContract = serde_json::from_str(
            r#"{"name":"x","chain_id":0,"version":"0.8.30","target":"a.sol",
                "sources":{"a.sol":"import \"./missing.sol\"; contract A {}"}}"#,
        )
        .unwrap();
        let mut summary = Summary::default();
        summary.add(&check(&record, Path::new("0_x.json"), false));
        let path = std::env::temp_dir().join(format!("sourcify-gate-{}.md", std::process::id()));
        let options = ReportOptions {
            report: Some(path.clone()),
            expected_failures: None,
            report_only: true,
            stale_check: false,
        };
        let expected: ExpectedFailures = toml::from_str("").unwrap();

        finish(&summary, &expected, &options).unwrap();
        let report = std::fs::read_to_string(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(report.contains("**Gate: failed.**"), "{report}");
        assert!(report.contains("- unexpected failures: bind:"), "{report}");
    }

    #[test]
    fn testdata_corpus_passes_every_check() {
        let corpus = Corpus::open(&testdata_corpus_dir()).unwrap();
        for path in corpus.shard(1, 0) {
            let outcome = check(&corpus::read_contract(path).unwrap(), path, false);
            assert!(outcome.passed(), "{path:?}: {outcome:?}");
        }
    }

    #[test]
    fn diagnostics_land_in_their_check() {
        let record: CorpusContract = serde_json::from_str(
            r#"{"name":"x","chain_id":0,"version":"0.8.30","target":"a.sol",
                "sources":{"a.sol":"pragma solidity ^0.8.0; import \"./missing.sol\"; contract A { function f() public { return undefinedName; } }"}}"#,
        )
        .unwrap();
        let outcome = check(&record, Path::new("0_x.json"), false);
        let keys: Vec<String> = outcome.failures.iter().map(outcome::Failure::key).collect();
        assert_eq!(
            outcome
                .skipped_checks
                .get("storage_layout")
                .map(String::as_str),
            Some("no artifacts")
        );
        assert!(keys.iter().any(|key| key.starts_with("bind:")), "{keys:?}");
        assert_eq!(outcome.evm_target.as_deref(), Some("Prague"));
    }

    #[test]
    fn storage_mismatches_count_once_per_contract_and_code() {
        let record: CorpusContract = serde_json::from_str(
            r#"{"name":"x","chain_id":0,"version":"0.8.30","target":"a.sol",
                "sources":{"a.sol":"contract A { uint256 a; int256 b; }"},
                "artifacts":{"storageLayout":{
                    "storage":[{"label":"a","offset":0,"slot":"0","type":"t_x"},
                               {"label":"b","offset":0,"slot":"1","type":"t_y"}],
                    "types":{"t_x":{"encoding":"inplace","label":"uint128","numberOfBytes":"32"},
                             "t_y":{"encoding":"inplace","label":"int128","numberOfBytes":"32"}}}}}"#,
        )
        .unwrap();
        let outcome = check(&record, Path::new("0_x.json"), false);
        let failures: Vec<(String, usize)> = outcome
            .failures
            .iter()
            .map(|failure| (failure.key(), failure.count))
            .collect();
        assert_eq!(failures, [("storage_types:[*].type.label".to_owned(), 2)]);
    }

    #[test]
    fn storage_types_are_compared_through_nested_types() {
        let path = testdata_corpus_dir().join("contracts/0_storage_types.json");
        let mut record = corpus::read_contract(&path).unwrap();
        let types = record
            .artifacts
            .as_mut()
            .unwrap()
            .pointer_mut("/storageLayout/types")
            .unwrap();
        // Each entry is only reachable through an array base, a mapping value or a
        // struct member, the last one through a recursive struct.
        types["t_struct(Inner)16_storage"]["members"][2]["offset"] = 20.into();
        types["t_struct(Node)37_storage"]["members"][1]["slot"] = "2".into();
        types["t_array(t_uint256)dyn_storage"]["encoding"] = "inplace".into();
        types["t_uint16"]["label"] = "uint8".into();
        types["t_array(t_struct(Node)37_storage)dyn_storage"]["numberOfBytes"] = "64".into();

        let outcome = check(&record, &path, false);
        let keys: Vec<String> = outcome.failures.iter().map(outcome::Failure::key).collect();
        assert_eq!(
            keys,
            [
                "storage_types:[*].type.encoding",
                "storage_types:[*].type.label",
                "storage_types:[*].type.members.offset",
                "storage_types:[*].type.members.slot",
                "storage_types:[*].type.numberOfBytes",
            ]
        );
    }

    #[test]
    fn unsupported_versions_and_targets_are_skips() {
        let mut record: CorpusContract = serde_json::from_str(
            r#"{"name":"x","chain_id":0,"version":"0.7.6","target":"a.sol","sources":{"a.sol":"contract A {}"}}"#,
        )
        .unwrap();
        assert_eq!(
            check(&record, Path::new("0_x.json"), false)
                .skipped
                .as_deref(),
            Some("unsupported language version 0.7.6")
        );
        record.version = "0.8.30".to_owned();
        record.evm_version = Some("nonesuch".to_owned());
        assert_eq!(
            check(&record, Path::new("0_x.json"), false)
                .skipped
                .as_deref(),
            Some("unknown EVM target nonesuch")
        );
    }
}
