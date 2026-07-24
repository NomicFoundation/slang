use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

const CRATE_NAME: &str = "solidity_testing_solc_comparison";
const EXPECTED_FAILURES_FILE: &str = "expected-failures.json";

/// The set of tests expected to currently fail, grouped by the Solidity version
/// they fail at.
#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Baseline {
    failures: SortedMap<LanguageVersion, SortedSet<String>>,
}

impl Baseline {
    /// Loads the checked-in baseline.
    pub fn load() -> Result<Self> {
        let path = expected_failures_path()?;
        let contents = path
            .read_to_string()
            .with_context(|| format!("failed to read the checked-in baseline at {path:?}"))?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Whether `test_path` is expected to fail at `version`.
    pub fn is_expected_failure(&self, version: LanguageVersion, test_path: &str) -> bool {
        self.failures
            .get(&version)
            .is_some_and(|paths| paths.contains(test_path))
    }

    /// Records that `test_path` did or didn't fail at `version`, updating the
    /// in-memory set accordingly, and — when that actually changes the baseline
    /// — writing it straight back to disk under a file lock.
    pub fn record(
        &mut self,
        version: LanguageVersion,
        test_path: &str,
        failed: bool,
    ) -> Result<()> {
        let changed = if failed {
            self.failures
                .entry(version)
                .or_default()
                .insert(test_path.to_owned())
        } else if let Some(paths) = self.failures.get_mut(&version) {
            let changed = paths.remove(test_path);
            if paths.is_empty() {
                self.failures.remove(&version);
            }
            changed
        } else {
            false
        };

        if changed {
            self.write_locked()?;
        }
        Ok(())
    }

    /// Serializes the baseline to `expected-failures.json`, holding an exclusive
    /// file lock across the truncate-and-rewrite (the same `File::lock` the solc
    /// binary cache uses) so a concurrent writer waits rather than corrupting
    /// the file. The lock is released when `file` is dropped.
    fn write_locked(&self) -> Result<()> {
        let path = expected_failures_path()?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("failed to open baseline for writing at {path:?}"))?;

        File::lock(&file).with_context(|| format!("failed to lock baseline at {path:?}"))?;

        let json = serde_json::to_string_pretty(self)?;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(json.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }
}

/// Path to the checked-in baseline, located via the shared cargo-workspace
/// helper (which resolves the crate's source directory from the workspace
/// manifest).
fn expected_failures_path() -> Result<PathBuf> {
    Ok(CargoWorkspace::locate_source_crate(CRATE_NAME)?.join(EXPECTED_FAILURES_FILE))
}
