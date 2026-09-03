use std::path::PathBuf;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

const CRATE_NAME: &str = "solidity_v2_testing_solc";
pub const RESULTS_FILE: &str = "results.generated.json";

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
    /// The failures we don't stand behind. Until
    /// [`expected_failures::split`] has run this holds *every* failing test in
    /// the run.
    ///
    /// [`expected_failures::split`]: crate::expected_failures::split
    pub unexpected_failures: Vec<Failure>,
    /// How many failures an [`expected_failures`] case accounts for. Counting
    /// them rather than keeping their paths.
    ///
    /// `None` until [`expected_failures::split`] has run.
    ///
    /// [`expected_failures`]: crate::expected_failures
    /// [`expected_failures::split`]: crate::expected_failures::split
    pub expected_failures: Option<usize>,
}

/// What a whole run produced, per version.
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
#[derive(Serialize, Deserialize)]
pub struct VersionResults {
    commit: String,
    executed: usize,
    passed: usize,
    expected_failures: usize,
    unexpected_failures: usize,
    unexpected_failures_paths: SortedSet<String>,
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
            .is_some_and(|results| results.unexpected_failures_paths.contains(test_path))
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

    /// Total tests slang rejected without an [`expected_failures`] entry
    /// covering them, across every version.
    ///
    /// [`expected_failures`]: crate::expected_failures
    pub fn unexpected_failures(&self) -> usize {
        self.versions
            .values()
            .map(|results| results.unexpected_failures)
            .sum()
    }

    /// Total tests slang rejected for an expected reason, across every version.
    pub fn expected_failures(&self) -> usize {
        self.versions
            .values()
            .map(|results| results.expected_failures)
            .sum()
    }
}

impl FromIterator<VersionRun> for TestResults {
    fn from_iter<I: IntoIterator<Item = VersionRun>>(runs: I) -> Self {
        let versions = runs
            .into_iter()
            .map(|run| {
                let unexpected_failures = run.unexpected_failures.len();
                let expected_failures = run
                    .expected_failures
                    .expect("expected failures should be set after splitting");

                let results = VersionResults {
                    commit: run.commit,
                    executed: run.executed,
                    passed: run.executed - unexpected_failures - expected_failures,
                    expected_failures,
                    unexpected_failures,
                    unexpected_failures_paths: run
                        .unexpected_failures
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
