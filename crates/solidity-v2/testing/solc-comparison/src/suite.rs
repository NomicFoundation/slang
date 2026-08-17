use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use rayon::prelude::*;

use crate::dataset::Datasets;
use crate::results::{Failure, TestResults, VersionRun};
use crate::runner::{self, Outcome};

/// Fetches every supported version's semantic tests, compiles all of them, and
/// writes the result out through [`CodegenFileSystem`] — which rewrites the
/// checked-in files locally, and asserts they still match in CI.
pub fn run() -> Result<()> {
    let datasets = Datasets::create()?;

    // Loaded before we write, so we can point at *why* anything newly fails
    // (the checked-in file only records which `(version, test)` pairs do).
    let previous = TestResults::load()?;

    let runs = execute(&datasets)?;

    // This step only prints new diagnostics, but the test fails or passes
    // on the write below, checking that the results stay the same.
    report_new_failures(&previous, &runs);

    let results: TestResults = runs.into_iter().collect();
    report_summary(&results);

    results.write(&mut CodegenFileSystem::default())
}

/// Compiles every test in every dataset.
fn execute(datasets: &Datasets) -> Result<Vec<VersionRun>> {
    // Roughly 1,600 tests across ~37 versions, each entirely independent, so
    // the whole matrix fans out across rayon.
    datasets
        .versions()
        .par_iter()
        .map(|dataset| {
            let version = dataset.version();
            let test_files = dataset.test_files()?;

            let failures: Vec<Failure> = test_files
                .par_iter()
                .filter_map(
                    |test_file| match runner::run_test(&test_file.path, version) {
                        Outcome::Passed => None,
                        Outcome::Failed { diagnostics } => Some(Failure {
                            version,
                            test_path: test_file.relative_path.clone(),
                            diagnostics,
                        }),
                    },
                )
                .collect();

            Ok(VersionRun {
                version,
                commit: dataset.commit_sha().to_owned(),
                executed: test_files.len(),
                failures,
            })
        })
        .collect()
}

/// Prints what the run covered. Under `nextest` a passing test's output is
/// captured, so this shows up on failure or with `--no-capture`; the same
/// numbers are recorded per version in the checked-in results either way.
fn report_summary(results: &TestResults) {
    println!(
        "Compiled {executed} semantic test(s): {passed} passed, {failed} failed.",
        executed = results.executed(),
        passed = results.passed(),
        failed = results.failed(),
    );
}

/// How many newly-failing tests to print diagnostics for. A regression usually
/// has one root cause, so the first few are enough to work from; the diff
/// against the checked-in file still names every one of them.
const MAX_REPORTED_FAILURES: usize = 10;

/// Prints the diagnostics behind each failure that isn't already checked in.
///
/// Note that an old failure that is now passing is not reported here.
fn report_new_failures(previous: &TestResults, runs: &[VersionRun]) {
    let new_failures: Vec<&Failure> = runs
        .iter()
        .flat_map(|run| &run.failures)
        .filter(|failure| !previous.contains_failure(failure.version, &failure.test_path))
        .collect();

    if new_failures.is_empty() {
        return;
    }

    println!(
        "slang rejected {count} test(s) that are not checked in as failing:",
        count = new_failures.len()
    );

    for failure in new_failures.iter().take(MAX_REPORTED_FAILURES) {
        println!(
            "\n=== {version} | {test_path}\n{diagnostics}",
            version = failure.version,
            test_path = failure.test_path,
            diagnostics = failure.diagnostics.join("\n"),
        );
    }

    if let Some(remaining) = new_failures
        .len()
        .checked_sub(MAX_REPORTED_FAILURES)
        .filter(|remaining| *remaining > 0)
    {
        println!("\n... and {remaining} more (see the diff below).");
    }
}
