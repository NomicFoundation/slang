use std::path::Path;

use anyhow::{Context, Result, ensure};
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};
use solidity_v2_testing_utils::evm_targets::default_evm_target;

use crate::snapshots::config::TestCase;
use crate::snapshots::config::raw::test_config::RawTestConfig;

/// The name of the per-test configuration file.
const CONFIG_FILE_NAME: &str = ".tests.config.json";

/// Resolved test configuration. Every test runs at all language versions,
/// analyzing each at the EVM target `solc` of that version defaults to, unless
/// the config narrows either axis.
#[derive(Default)]
pub struct TestConfig {
    /// Narrows the language versions the test runs at. This governs the whole
    /// run: both the slang and the solc side only ever see these versions.
    pub(super) override_language_versions: Option<LanguageVersionSpecifier>,

    /// Pins every language version to this EVM target, instead of the one its
    /// own `solc` defaults to.
    pub(super) override_evm_target: Option<EvmTarget>,

    /// The language versions where slang and solc are expected to disagree on
    /// the status (success/failure) of a snapshot. Used for diagnostics where
    /// slang intentionally diverges from solc.
    pub(super) expected_solc_divergence: Option<Vec<LanguageVersionSpecifier>>,
}

impl TestConfig {
    /// Resolves the config for a test rooted at `test_dir`, by visiting the
    /// `.tests.config.json` in `test_dir` and then in each parent directory,
    /// up to and including the owning crate root (the nearest ancestor
    /// containing a `Cargo.toml`).
    ///
    /// Every field is resolved independently, with the closest config file
    /// providing it winning. This way, nested configs only need to override the
    /// individual fields they care about, and a test without any config file
    /// in its chain gets the defaults.
    pub fn resolve(test_dir: &Path) -> Result<Self> {
        let mut current_dir = test_dir;
        let mut resolved = Self::default();

        loop {
            let config_path = current_dir.join(CONFIG_FILE_NAME);

            if config_path.exists() {
                ensure!(config_path.is_file(), "`{CONFIG_FILE_NAME}` is not a file");

                let contents = config_path.read_to_string()?;
                let raw: RawTestConfig = serde_json::from_str(&contents)
                    .with_context(|| format!("Failed to parse test config: {config_path:?}"))?;

                let config = Self::try_from(raw)
                    .with_context(|| format!("Invalid test config: {config_path:?}"))?;

                resolved.absorb(config);
            }

            // Search only within the owning crate: stop once we reach the
            // directory holding its `Cargo.toml`.
            if current_dir.join("Cargo.toml").exists() {
                break;
            }

            current_dir = current_dir.unwrap_parent();
        }

        resolved.validate().with_context(|| {
            format!("Failed to resolve `{CONFIG_FILE_NAME}` for test directory {test_dir:?}.")
        })
    }

    /// The cells the test runs, in order: every selected language version, at
    /// the target it is analyzed at.
    pub fn test_cases(&self) -> impl Iterator<Item = TestCase> + '_ {
        LanguageVersion::ALL
            .iter()
            .copied()
            .filter(|version| {
                self.override_language_versions
                    .as_ref()
                    .is_none_or(|specifier| specifier.contains(*version))
            })
            .map(|language_version| TestCase {
                language_version,
                evm_target: self
                    .override_evm_target
                    .unwrap_or_else(|| default_evm_target(language_version)),
                expected_solc_divergence: self.expected_solc_divergence.as_ref().is_some_and(
                    |entries| {
                        entries
                            .iter()
                            .any(|specifier| specifier.contains(language_version))
                    },
                ),
            })
    }

    /// Absorbs fields from `parent`, a config file further up the traversal, for
    /// every field that `self` doesn't already provide.
    fn absorb(&mut self, parent: Self) {
        let Self {
            override_language_versions: parent_override_language_versions,
            override_evm_target: parent_override_evm_target,
            expected_solc_divergence: parent_expected_solc_divergence,
        } = parent;

        self.override_language_versions = self
            .override_language_versions
            .take()
            .or(parent_override_language_versions);

        self.override_evm_target = self
            .override_evm_target
            .take()
            .or(parent_override_evm_target);

        self.expected_solc_divergence = self
            .expected_solc_divergence
            .take()
            .or(parent_expected_solc_divergence);
    }

    fn validate(self) -> Result<Self> {
        let entries = self.expected_solc_divergence.as_deref().unwrap_or_default();

        for (index, entry) in entries.iter().enumerate() {
            // Check if an entry is too wide for the tested language versions:
            if let Some(versions) = self.override_language_versions.as_ref() {
                ensure!(
                    entry.intersect(versions) == entry.intersect(entry),
                    "`expected_solc_divergence` entry '{entry:?}' is not inside \
                     `override_language_versions` '{versions:?}'."
                );
            }

            // Check for overlap between different entries:
            for other in &entries[index + 1..] {
                ensure!(
                    entry.intersect(other).is_none(),
                    "`expected_solc_divergence` entries '{entry:?}' and '{other:?}' overlap."
                );
            }
        }

        Ok(self)
    }
}
