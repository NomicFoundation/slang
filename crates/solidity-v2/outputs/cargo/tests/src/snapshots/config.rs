use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
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
    pub matrix: TestMatrix,
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
    /// Resolves the config for a test rooted at `test_dir`, by searching for a
    /// `.tests.config.json` in `test_dir` and then in each parent directory,
    /// up to and including the owning crate root (the nearest ancestor
    /// containing a `Cargo.toml`).
    pub fn resolve(test_dir: &Path) -> Result<Self> {
        let mut current_dir = test_dir;

        loop {
            let config_path = current_dir.join(CONFIG_FILE_NAME);

            if config_path.exists() {
                ensure!(config_path.is_file(), "`{CONFIG_FILE_NAME}` is not a file");

                let contents = config_path.read_to_string()?;
                let raw: RawConfigFile = serde_json::from_str(&contents)
                    .with_context(|| format!("Failed to parse test config: {config_path:?}"))?;

                return Ok(raw.into());
            }

            // Search only within the owning crate: stop once we reach the
            // directory holding its `Cargo.toml`, failing if nothing was found.
            if current_dir.join("Cargo.toml").exists() {
                break;
            }

            current_dir = current_dir.unwrap_parent();
        }

        bail!(
            "No `{CONFIG_FILE_NAME}` found for test directory {test_dir:?} within its crate. \
             Each test suite must define one at its root directory."
        );
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfigFile {
    matrix: RawTestMatrix,
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

impl From<RawConfigFile> for TestConfig {
    fn from(raw: RawConfigFile) -> Self {
        let matrix = match raw.matrix {
            RawTestMatrix::SingleVersionAllTargets { version, reason } => {
                assert!(!reason.trim().is_empty(), "Reason must be non-empty");
                TestMatrix::SingleVersionAllTargets {
                    version: version.try_into().unwrap(),
                }
            }
            RawTestMatrix::SingleTargetAllVersions { target, reason } => {
                assert!(!reason.trim().is_empty(), "Reason must be non-empty");
                TestMatrix::SingleTargetAllVersions {
                    target: target.as_str().try_into().unwrap(),
                }
            }
        };

        Self { matrix }
    }
}
