use std::path::Path;

use anyhow::{Context, Result, ensure};
use infra_utils::paths::PathExtensions;
use serde::Deserialize;

use super::test_matrix::{RawTestMatrix, TestMatrix};

/// The name of the per-test configuration file.
const CONFIG_FILE_NAME: &str = ".tests.config.json";

/// Resolved test configuration.
#[derive(Clone, Debug)]
pub struct TestConfig {
    /// Declares how the test iterates over the `LanguageVersion`/`EvmTarget` matrix.
    /// Exactly one axis varies per test — the other is pinned by the config.
    pub matrix: TestMatrix,
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
        let mut resolved = RawTestConfig::default();

        loop {
            let config_path = current_dir.join(CONFIG_FILE_NAME);

            if config_path.exists() {
                ensure!(config_path.is_file(), "`{CONFIG_FILE_NAME}` is not a file");

                let contents = config_path.read_to_string()?;
                let raw: RawTestConfig = serde_json::from_str(&contents)
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
struct RawTestConfig {
    /// Every field is optional: values not provided here are inherited from the
    /// closest parent directory that does provide them.
    matrix: Option<RawTestMatrix>,
}

impl RawTestConfig {
    /// Absorbs fields from `parent`, a config file further up the traversal, for
    /// every field that `self` doesn't already provide.
    fn absorb(&mut self, parent: Self) {
        let Self {
            matrix: parent_matrix,
        } = parent;

        if let Some(parent_matrix) = parent_matrix {
            if let Some(matrix) = self.matrix.as_mut() {
                matrix.absorb(parent_matrix);
            } else {
                self.matrix = Some(parent_matrix);
            }
        }
    }
}

impl TryFrom<RawTestConfig> for TestConfig {
    type Error = anyhow::Error;

    fn try_from(raw: RawTestConfig) -> Result<Self> {
        let RawTestConfig { matrix } = raw;

        Ok(Self {
            matrix: matrix
                .context("No config file provides the `matrix` field.")?
                .try_into()
                .context("Invalid `matrix` field.")?,
        })
    }
}
