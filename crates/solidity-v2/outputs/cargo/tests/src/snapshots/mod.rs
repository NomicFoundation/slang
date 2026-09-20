//! Shared snapshot driver for v2 cargo test runners.

mod config;

use std::path::Path;

use anyhow::Result;
pub use config::{TestCase, TestConfig};
use infra_utils::codegen::CodegenFileSystem;
use slang_solidity_v2_common::diagnostics::{DiagnosticCollection, DiagnosticSeverity};
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
    pub fn from_diagnostics(diagnostics: &DiagnosticCollection) -> Self {
        match diagnostics.highest_severity() {
            Some(DiagnosticSeverity::Error) => Self::Failure,
            Some(DiagnosticSeverity::Warning) => Self::Warning,
            None => Self::Success,
        }
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

/// Drives a runner over every [`TestCase`] of the test, in order.
///
/// A snapshot file is written only when its contents differ from the previous
/// version's, so each file marks the version where the output changed.
pub fn generate_snapshots<F>(
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    test_cases: &[TestCase],
    output_subdir: &str,
    mut run: F,
) -> Result<Vec<SnapshotOutcome>>
where
    F: FnMut(LanguageVersion, EvmTarget) -> Result<SnapshotOutcome>,
{
    let mut outcomes = Vec::new();
    let mut last_contents: Option<String> = None;

    for case in test_cases {
        let (version, target) = (case.language_version, case.evm_target);
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
