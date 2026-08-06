use anyhow::{Context, Result};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::generated_file;

const FAILURES_FILE: &str = "failures.generated.json";

/// A single `(version, test)` pair that slang didn't compile cleanly.
pub struct Failure {
    pub version: LanguageVersion,
    /// The test's path relative to `semanticTests`.
    pub test_path: String,
    pub diagnostics: Vec<String>,
}

/// The tests that don't compile cleanly, grouped by the Solidity version they
/// fail at. This is what a run actually produces; checking it in is what turns
/// it into the baseline the next run is held to.
#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Failures {
    tests: SortedMap<LanguageVersion, SortedSet<String>>,
}

impl Failures {
    /// Loads the checked-in set. A missing file is empty rather than an error:
    /// it only feeds the reporting below, and [`Self::write`] is what decides
    /// whether the file being absent is a problem.
    pub fn load() -> Result<Self> {
        let path = generated_file(FAILURES_FILE)?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = path
            .read_to_string()
            .with_context(|| format!("Failed to read the checked-in failures at {path:?}"))?;

        Ok(serde_json::from_str(&contents)?)
    }

    /// Whether `test_path` is recorded as failing at `version`.
    pub fn contains(&self, version: LanguageVersion, test_path: &str) -> bool {
        self.tests
            .get(&version)
            .is_some_and(|paths| paths.contains(test_path))
    }

    /// Writes this run's result out. Locally that rewrites the checked-in file;
    /// in CI it asserts the two match, which catches drift in either direction
    /// — a fresh failure is a regression, and a recorded case that now passes
    /// means the file is stale.
    pub fn write(&self, fs: &mut CodegenFileSystem) -> Result<()> {
        fs.write_file_formatted(generated_file(FAILURES_FILE)?, serde_json::to_string(self)?)
    }
}

impl FromIterator<Failure> for Failures {
    fn from_iter<I: IntoIterator<Item = Failure>>(failures: I) -> Self {
        let mut tests: SortedMap<LanguageVersion, SortedSet<String>> = SortedMap::new();

        for failure in failures {
            tests
                .entry(failure.version)
                .or_default()
                .insert(failure.test_path);
        }

        Self { tests }
    }
}
