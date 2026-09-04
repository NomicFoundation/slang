//! Shared snapshot driver for v2 cargo test runners.

mod config;

use std::path::Path;

use anyhow::Result;
pub use config::{TestConfig, TestMatrix};
use infra_utils::codegen::CodegenFileSystem;
use slang_solidity_v2_common::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use strum::Display;

/// Result of running a single iteration of a snapshot test.
/// Ordered by severity (highest to lowest).
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "kebab-case")]
pub enum SnapshotStatus {
    Failure,
    Warning,
    Success,
}

impl SnapshotStatus {
    pub fn from_diagnostics<'a>(diagnostics: impl IntoIterator<Item = &'a Diagnostic>) -> Self {
        let mut status = Self::Success;

        for diagnostic in diagnostics {
            match diagnostic.severity() {
                DiagnosticSeverity::Error => return Self::Failure,
                DiagnosticSeverity::Warning => status = Self::Warning,
            }
        }

        status
    }
}

#[derive(Clone, Debug)]
pub struct SnapshotOutcome {
    pub version: LanguageVersion,
    pub target: EvmTarget,

    pub status: SnapshotStatus,
    pub contents: String,
    pub extension: &'static str,
}

/// Drives a runner that takes both `LanguageVersion` and `EvmTarget`. The
/// helper iterates whichever axis the config varies, pinning the other.
pub fn generate_snapshots<F>(
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    config: &TestConfig,
    output_subdir: &str,
    run: F,
) -> Result<Vec<SnapshotOutcome>>
where
    F: FnMut(LanguageVersion, EvmTarget) -> Result<SnapshotOutcome>,
{
    match &config.matrix {
        TestMatrix::SingleTargetAllVersions(matrix) => {
            iterate_versions(test_dir, fs, output_subdir, matrix.target, run)
        }
        TestMatrix::SingleVersionAllTargets(matrix) => {
            iterate_targets(test_dir, fs, output_subdir, matrix.version, run)
        }
    }
}

fn iterate_versions<F>(
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    output_subdir: &str,
    target: EvmTarget,
    mut run: F,
) -> Result<Vec<SnapshotOutcome>>
where
    F: FnMut(LanguageVersion, EvmTarget) -> Result<SnapshotOutcome>,
{
    let mut outcomes = Vec::with_capacity(LanguageVersion::ALL.len());
    let mut last_contents: Option<String> = None;

    for &version in LanguageVersion::ALL {
        let outcome = run(version, target)?;

        if last_contents.as_ref() != Some(&outcome.contents) {
            let filename = format!(
                "{version}-{status}.{extension}",
                status = outcome.status,
                extension = outcome.extension,
            );

            let output_path = test_dir.join(output_subdir).join(filename);
            fs.write_file_raw(&output_path, &outcome.contents)?;

            last_contents = Some(outcome.contents.clone());
        }

        outcomes.push(outcome);
    }

    Ok(outcomes)
}

fn iterate_targets<F>(
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    output_subdir: &str,
    version: LanguageVersion,
    mut run: F,
) -> Result<Vec<SnapshotOutcome>>
where
    F: FnMut(LanguageVersion, EvmTarget) -> Result<SnapshotOutcome>,
{
    let mut outcomes = Vec::with_capacity(EvmTarget::ALL.len());
    let mut last_contents: Option<String> = None;

    for &target in EvmTarget::ALL {
        let outcome = run(version, target)?;

        if last_contents.as_ref() != Some(&outcome.contents) {
            let index = target as u32;
            let name = target.to_string().to_lowercase();
            let filename = format!(
                "{index:02}-{name}-{status}.{extension}",
                status = outcome.status,
                extension = outcome.extension,
            );

            let output_path = test_dir.join(output_subdir).join(filename);
            fs.write_file_raw(&output_path, &outcome.contents)?;

            last_contents = Some(outcome.contents.clone());
        }

        outcomes.push(outcome);
    }

    Ok(outcomes)
}
