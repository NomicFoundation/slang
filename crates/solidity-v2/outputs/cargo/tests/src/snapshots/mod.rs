//! Shared snapshot driver for v2 cargo test runners.

mod config;

use std::path::Path;

use anyhow::Result;
pub use config::{TestConfig, TestMatrix};
use infra_utils::codegen::CodegenFileSystem;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

/// Result of running a single iteration of a snapshot test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotStatus {
    Success,
    Failure,
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
    output_subdir: &str,
    run: F,
) -> Result<Vec<SnapshotOutcome>>
where
    F: FnMut(LanguageVersion, EvmTarget) -> Result<SnapshotOutcome>,
{
    let config = TestConfig::resolve(test_dir)?;

    match config.matrix {
        TestMatrix::SingleTargetAllVersions { target } => {
            iterate_versions(test_dir, fs, output_subdir, target, run)
        }
        TestMatrix::SingleVersionAllTargets { version } => {
            iterate_targets(test_dir, fs, output_subdir, version, run)
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
                status = match outcome.status {
                    SnapshotStatus::Success => "success",
                    SnapshotStatus::Failure => "failure",
                },
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
                status = match outcome.status {
                    SnapshotStatus::Success => "success",
                    SnapshotStatus::Failure => "failure",
                },
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
