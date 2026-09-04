mod slang;
mod solc;

use anyhow::Result;
pub(crate) use slang::SlangTarget;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
pub(crate) use solc::SolcTarget;

use crate::snapshots::SnapshotStatus;

/// The outcome of running a target on a single input.
pub(crate) struct TargetOutcome {
    /// Every diagnostic, rendered for the snapshot body, regardless of
    /// severity.
    pub diagnostics: Vec<String>,

    /// Status determined by the highest severity reported by the target.
    /// Used for comparing outcomes between different targets.
    pub status: SnapshotStatus,
}

pub(crate) trait TestTarget {
    fn name(&self) -> &'static str;

    fn compile(
        &self,
        files: &SortedMap<FileId, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<TargetOutcome>;
}
