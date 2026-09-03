use anyhow::{Result, bail};
use infra_utils::codegen::CodegenFileSystem;
use rayon::prelude::*;

use crate::dataset::Datasets;
use crate::expected_failures;
use crate::results::{AllFailures, Failure, SplitFailures, TestResults, VersionRun};
use crate::runner::{self, Outcome};

/// Fetches every supported version's semantic tests, compiles all of them,
/// writes the result out through [`CodegenFileSystem`] — which rewrites the
/// checked-in file locally, and asserts it still matches in CI — and then holds
/// the run to it: any test slang rejected that isn't an expected failure fails
/// this suite.
pub fn run() -> Result<()> {
    let datasets = Datasets::create()?;

    // Loaded before we write, so we can point at *why* anything newly fails
    // (the checked-in file only records which `(version, test)` pairs do).
    let previous = TestResults::load()?;

    let runs = execute(&datasets)?;

    // Split the runs into those that match the expected failures
    // and those that don't.
    let (runs, stale_check) = expected_failures::split_and_check(runs);

    report_new_failures(&previous, &runs);

    // Rendered while the runs are still around: `TestResults` keeps only the
    // paths, and the diagnostics behind them are what the report needs.
    let unexpected = render_unexpected_failures(&runs);

    let results: TestResults = runs.into_iter().collect();
    report_summary(&results);

    // Written before any error is reported.
    results.write(&mut CodegenFileSystem::default())?;

    // Errors are reported at the end of the function, to
    // guarantee the snapshot test file is written.
    stale_check?;

    if !unexpected.is_empty() {
        bail!(
            "slang rejected {count} semantic test(s) that `solc` compiles. Each \
             one is either a gap to fix, or a difference we stand behind and \
             should declare in `src/expected_failures.rs`:\n\n{failures}",
            count = unexpected.len(),
            failures = unexpected.join("\n\n"),
        );
    }

    Ok(())
}

/// Renders one block per unexpected failure: the version it ran at, the test's
/// path, and the diagnostics slang reported for it.
fn render_unexpected_failures(runs: &[VersionRun<SplitFailures>]) -> Vec<String> {
    runs.iter()
        .flat_map(|run| &run.failures.unexpected)
        .map(|failure| {
            format!(
                "- [{version}] {test_path}\n{diagnostics}",
                version = failure.version,
                test_path = failure.test_path,
                diagnostics = failure.diagnostics.join("\n"),
            )
        })
        .collect()
}

/// Compiles every test in every dataset.
fn execute(datasets: &Datasets) -> Result<Vec<VersionRun<AllFailures>>> {
    // Roughly 1,600 tests across ~37 versions, each entirely independent, so
    // the whole matrix fans out across rayon.
    datasets
        .versions()
        .par_iter()
        .map(|dataset| {
            let version = dataset.version();
            let test_files = dataset.test_files()?;

            let outcomes: Vec<Option<Failure>> = test_files
                .par_iter()
                .map(|test_file| {
                    let failure = match runner::run_test(&test_file.path, version)? {
                        Outcome::Passed => None,
                        Outcome::Failed { diagnostics } => Some(Failure {
                            version,
                            test_path: test_file.relative_path.clone(),
                            diagnostics,
                        }),
                    };

                    Ok(failure)
                })
                .collect::<Result<_>>()?;

            Ok(VersionRun {
                version,
                commit: dataset.commit_sha().to_owned(),
                executed: test_files.len(),
                failures: AllFailures(outcomes.into_iter().flatten().collect()),
            })
        })
        .collect()
}

/// Prints what the run covered. Under `nextest` a passing test's output is
/// captured, so this shows up on failure or with `--no-capture`; the same
/// numbers are recorded per version in the checked-in results either way.
fn report_summary(results: &TestResults) {
    println!(
        "Compiled {executed} semantic test(s): {passed} passed, \
         {unexpected} unexpected failure(s), {expected} expected.",
        executed = results.executed(),
        passed = results.passed(),
        unexpected = results.unexpected_failures(),
        expected = results.expected_failures(),
    );
}

/// How many newly-failing tests to print diagnostics for. A regression usually
/// has one root cause, so the first few are enough to work from; the diff
/// against the checked-in file still names every one of them.
const MAX_REPORTED_FAILURES: usize = 10;

/// Prints the diagnostics behind each failure that isn't already checked in.
///
/// Note that an old failure that is now passing is not reported here.
fn report_new_failures(previous: &TestResults, runs: &[VersionRun<SplitFailures>]) {
    let new_failures: Vec<&Failure> = runs
        .iter()
        .flat_map(|run| &run.failures.unexpected)
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
