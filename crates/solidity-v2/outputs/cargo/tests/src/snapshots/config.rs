use std::path::Path;

use anyhow::{Context, Result, ensure};
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

/// The name of the per-test configuration file.
const CONFIG_FILE_NAME: &str = ".tests.config.json";

/// Resolved test configuration.
#[derive(Clone, Copy, Debug)]
pub struct TestConfig {
    /// Declares how the test iterates over the `LanguageVersion`/`EvmTarget` matrix.
    /// Exactly one axis varies per test — the other is pinned by the config.
    pub matrix: TestMatrix,

    /// Declares that slang and solc are expected to disagree on the status
    /// (success/failure) of at least one snapshot across versions/targets.
    /// Used for diagnostics where Slang is intentionally stricter than solc,
    /// like `pragma experimental SMTChecker` that we don't support.
    ///
    /// Defaults to `false` when the config file omits it. Tests fail both when
    /// an undeclared mismatch appears, or when a declared one no longer does.
    pub contains_status_mismatch: bool,
}

/// Configuration controlling how a snapshot test iterates over the
/// `LanguageVersion`/`EvmTarget` matrix. Exactly one axis varies per test —
/// the other is pinned by the config.
#[derive(Clone, Copy, Debug)]
pub enum TestMatrix {
    /// Pin the language version; iterate over all `EvmTarget` variants.
    SingleVersionAllTargets { version: LanguageVersion },
    /// Pin the EVM target; iterate over all `LanguageVersion` variants.
    SingleTargetAllVersions { target: EvmTarget },
}

impl TestConfig {
    /// Resolves the config for a test rooted at `test_dir`, by visiting the
    /// `.tests.config.json` in `test_dir` and then in each parent directory,
    /// up to and including the owning crate root (the nearest ancestor
    /// containing a `Cargo.toml`).
    ///
    /// Every field is resolved independently, with the closest config file
    /// providing it winning. This way, nested configs only need to override the
    /// individual fields they care about.
    pub fn resolve(test_dir: &Path) -> Result<Self> {
        let mut current_dir = test_dir;
        let mut resolved = RawConfigFile::default();

        loop {
            let config_path = current_dir.join(CONFIG_FILE_NAME);

            if config_path.exists() {
                ensure!(config_path.is_file(), "`{CONFIG_FILE_NAME}` is not a file");

                let contents = config_path.read_to_string()?;
                let raw: RawConfigFile = serde_json::from_str(&contents)
                    .with_context(|| format!("Failed to parse test config: {config_path:?}"))?;

                resolved.absorb(raw);
            }

            // Search only within the owning crate: stop once we reach the
            // directory holding its `Cargo.toml`, failing if anything is still missing.
            if current_dir.join("Cargo.toml").exists() {
                break;
            }

            current_dir = current_dir.unwrap_parent();
        }

        resolved.try_into().with_context(|| {
            format!(
                "Failed to resolve `{CONFIG_FILE_NAME}` for test directory {test_dir:?} within its \
                 crate. Each test suite must define the required fields at its root directory."
            )
        })
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfigFile {
    /// Every field is optional: values not provided here are inherited from the
    /// closest parent directory that does provide them.
    matrix: Option<RawTestMatrix>,
    contains_status_mismatch: Option<RawContainsStatusMismatch>,
}

impl RawConfigFile {
    /// Absorbs values from `other`, a config file further up the traversal, for
    /// every field that `self` doesn't already provide.
    fn absorb(&mut self, other: Self) {
        let Self {
            matrix,
            contains_status_mismatch,
        } = other;

        self.matrix = self.matrix.take().or(matrix);

        self.contains_status_mismatch = self
            .contains_status_mismatch
            .take()
            .or(contains_status_mismatch);
    }
}

impl TryFrom<RawConfigFile> for TestConfig {
    type Error = anyhow::Error;

    fn try_from(raw: RawConfigFile) -> Result<Self> {
        let RawConfigFile {
            matrix,
            contains_status_mismatch,
        } = raw;

        Ok(Self {
            matrix: matrix
                .context("No config file provides the `matrix` field.")?
                .try_into()
                .context("Invalid `matrix` field.")?,
            contains_status_mismatch: contains_status_mismatch
                .map(TryInto::try_into)
                .transpose()
                .context("Invalid `contains_status_mismatch` field.")?
                .unwrap_or_default(),
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
enum RawTestMatrix {
    SingleVersionAllTargets {
        version: Version,
        /// Reason for pinning this version (for documentation purposes).
        reason: String,
    },
    SingleTargetAllVersions {
        target: String,
        /// Reason for pinning this target (for documentation purposes).
        reason: String,
    },
}

impl TryFrom<RawTestMatrix> for TestMatrix {
    type Error = anyhow::Error;

    fn try_from(raw: RawTestMatrix) -> Result<Self> {
        match raw {
            RawTestMatrix::SingleVersionAllTargets { version, reason } => {
                ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

                Ok(Self::SingleVersionAllTargets {
                    version: version
                        .clone()
                        .try_into()
                        .with_context(|| format!("Unsupported language version: '{version}'"))?,
                })
            }
            RawTestMatrix::SingleTargetAllVersions { target, reason } => {
                ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

                Ok(Self::SingleTargetAllVersions {
                    target: target
                        .as_str()
                        .try_into()
                        .with_context(|| format!("Unrecognized EVM target: '{target}'"))?,
                })
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawContainsStatusMismatch {
    value: bool,
    reason: String,
}

impl TryFrom<RawContainsStatusMismatch> for bool {
    type Error = anyhow::Error;

    fn try_from(raw: RawContainsStatusMismatch) -> Result<Self> {
        let RawContainsStatusMismatch { value, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        Ok(value)
    }
}
