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

    /// The worst diagnostic the target reported, as the snapshot status it maps
    /// to. This is what the two targets are compared on, so a target that errors
    /// matches one that errors (however many warnings either piled on top),
    /// while one that only warns doesn't match one that stayed silent.
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
