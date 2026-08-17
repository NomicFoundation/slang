use std::path::PathBuf;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

const CRATE_NAME: &str = "solidity_testing_solc_comparison";
const RESULTS_FILE: &str = "results.generated.json";

/// A single `(version, test)` pair that slang didn't compile cleanly.
pub struct Failure {
    pub version: LanguageVersion,
    /// The test's path relative to `semanticTests`.
    pub test_path: String,
    pub diagnostics: Vec<String>,
}

/// Everything one version's run produced. Unlike [`VersionResults`] this also
/// carries the diagnostics behind each failure, which are reported but not
/// checked in — they're far too noisy for a file we diff.
pub struct VersionRun {
    pub version: LanguageVersion,
    /// The commit this version's release tag resolved to when it was fetched.
    pub commit: String,
    /// How many tests ran, whether they passed or not.
    pub executed: usize,
    pub failures: Vec<Failure>,
}

/// What a whole run produced, per version. Checking this in is what turns it
/// into the baseline the next run is held to.
#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TestResults {
    versions: SortedMap<LanguageVersion, VersionResults>,
}

/// One version's record: which commit its tests came from, and what happened
/// when we compiled them.
///
/// Pinning the commit is what keeps the tally meaningful — solc's release tags
/// are mutable in principle, so we record what each one actually resolved to.
/// If one is ever re-pointed, the change lands right next to the counts it
/// invalidates rather than in a separate file.
///
/// `executed` and `passed` are redundant with `failures`, but recording them
/// means the diff also catches the dataset itself changing size — a version
/// whose test count moves is worth noticing, and it would otherwise be
/// invisible whenever the new tests happen to pass.
#[derive(Serialize, Deserialize)]
pub struct VersionResults {
    commit: String,
    executed: usize,
    passed: usize,
    failed: usize,
    failures: SortedSet<String>,
}

/// Path to the checked-in results file, located via the shared
/// cargo-workspace helper (which resolves the crate's source directory from the
/// workspace manifest).
fn results_path() -> Result<PathBuf> {
    Ok(CargoWorkspace::locate_source_crate(CRATE_NAME)?.join(RESULTS_FILE))
}

impl TestResults {
    /// Loads the checked-in results.
    ///
    /// A file that is missing, unreadable, or written in an older shape counts
    /// as empty rather than an error: this only feeds the reporting, and
    /// [`Self::write`] is what decides whether what's on disk is acceptable.
    /// That also means a change to this format needs no migration — the next
    /// local run just rewrites it.
    pub fn load() -> Result<Self> {
        let path = results_path()?;

        Ok(path
            .read_to_string()
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default())
    }

    /// Whether `test_path` is recorded as failing at `version`.
    pub fn contains_failure(&self, version: LanguageVersion, test_path: &str) -> bool {
        self.versions
            .get(&version)
            .is_some_and(|results| results.failures.contains(test_path))
    }

    /// Writes this run's results out. Locally that rewrites the checked-in
    /// file; in CI it asserts the two match, which catches drift in either
    /// direction — a fresh failure is a regression, and a recorded case that
    /// now passes means the file is stale.
    pub fn write(&self, fs: &mut CodegenFileSystem) -> Result<()> {
        fs.write_file_formatted(results_path()?, serde_json::to_string(self)?)
    }

    /// Total tests run across every version.
    pub fn executed(&self) -> usize {
        self.versions.values().map(|results| results.executed).sum()
    }

    /// Total tests slang compiled cleanly, across every version.
    pub fn passed(&self) -> usize {
        self.versions.values().map(|results| results.passed).sum()
    }

    /// Total tests slang rejected, across every version.
    pub fn failed(&self) -> usize {
        self.versions.values().map(|results| results.failed).sum()
    }
}

impl FromIterator<VersionRun> for TestResults {
    fn from_iter<I: IntoIterator<Item = VersionRun>>(runs: I) -> Self {
        let versions = runs
            .into_iter()
            .map(|run| {
                let failed = run.failures.len();

                let results = VersionResults {
                    commit: run.commit,
                    executed: run.executed,
                    passed: run.executed - failed,
                    failed,
                    failures: run
                        .failures
                        .into_iter()
                        .map(|failure| failure.test_path)
                        .collect(),
                };

                (run.version, results)
            })
            .collect();

        Self { versions }
    }
}
