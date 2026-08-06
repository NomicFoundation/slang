use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use rayon::prelude::*;

use crate::dataset::{self, Dataset};
use crate::failures::{Failure, Failures};
use crate::runner::{self, Outcome};

/// Fetches every supported version's semantic tests, compiles all of them, and
/// writes the result out through [`CodegenFileSystem`] — which rewrites the
/// checked-in files locally, and asserts they still match in CI.
pub fn run() -> Result<()> {
    let mut fs = CodegenFileSystem::default();

    let datasets = dataset::fetch_all_versions(&mut fs)?;

    // Loaded before we write, so we can point at *why* anything newly fails
    // (the checked-in file only records which `(version, test)` pairs do).
    let previous = Failures::load()?;

    let failures = collect_failures(&datasets)?;

    // This step only prints new diagnostics, but the test fails or passes
    // on the write below, checking that the list of failures stays the same.
    report_new_failures(&previous, &failures);

    failures.into_iter().collect::<Failures>().write(&mut fs)
}

/// Compiles every test in every dataset, keeping the ones slang rejected.
fn collect_failures(datasets: &[Dataset]) -> Result<Vec<Failure>> {
    // Roughly 1,600 tests across ~37 versions, each entirely independent, so
    // the whole matrix fans out across rayon.
    let per_version = datasets
        .par_iter()
        .map(|dataset| {
            let version = dataset.version();

            let failures: Vec<Failure> = dataset
                .test_files()?
                .into_par_iter()
                .filter_map(
                    |test_file| match runner::run_test(&test_file.path, version) {
                        Outcome::Passed => None,
                        Outcome::Failed { diagnostics } => Some(Failure {
                            version,
                            test_path: test_file.relative_path,
                            diagnostics,
                        }),
                    },
                )
                .collect();

            Ok(failures)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(per_version.into_iter().flatten().collect())
}

/// How many newly-failing tests to print diagnostics for. A regression usually
/// has one root cause, so the first few are enough to work from; the diff
/// against the checked-in file still names every one of them.
const MAX_REPORTED_FAILURES: usize = 10;

/// Prints the diagnostics behind each failure that isn't already checked in.
///
/// Note that an old failure that is now passing is not reported here.
fn report_new_failures(previous: &Failures, failures: &[Failure]) {
    let new_failures: Vec<&Failure> = failures
        .iter()
        .filter(|failure| !previous.contains(failure.version, &failure.test_path))
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
