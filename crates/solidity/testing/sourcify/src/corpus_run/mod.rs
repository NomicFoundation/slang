//! `run-corpus`: compiles every contract of a corpus snapshot with Slang v2 and
//! classifies what it reports; `report`: the same census from saved results.

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
use expected::ExpectedFailures;
use infra_utils::terminal::Terminal;
use outcome::{Outcome, classify};
use rayon::prelude::*;
use report::Summary;
use slang_solidity_v2::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};

use crate::command::{ReportCommand, ReportOptions, RunCorpusCommand};
use crate::corpus::{self, Corpus, CorpusContract};

/// The in-repo corpus of hand-written test cases, run by unit tests and the PR workflow.
#[cfg(test)]
pub fn testdata_corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata/corpus")
}

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
        if n % 1000 == 0 || n == total {
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
    let markdown = summary.markdown(expected);
    println!("{markdown}");
    if let Some(path) = &options.report {
        std::fs::write(path, &markdown).with_context(|| format!("Could not write {path:?}"))?;
    }
    match summary.gate(expected, options.stale_check).verdict() {
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
        classify(
            unit.diagnostics()
                .iter()
                .map(|diagnostic| diagnostic.kind()),
        )
    }));
    outcome.ms = started.elapsed().as_millis();
    match result {
        Ok((failures, warnings)) => {
            outcome.failures = failures;
            outcome.warnings = warnings;
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
        assert!(keys.iter().any(|key| key.starts_with("bind:")), "{keys:?}");
        assert_eq!(outcome.evm_target.as_deref(), Some("Prague"));
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
