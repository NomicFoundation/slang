mod slang;
mod solc;

use anyhow::Result;
pub(crate) use slang::SlangTarget;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
pub(crate) use solc::SolcTarget;

pub(crate) trait TestTarget {
    fn name(&self) -> &'static str;

    fn collect_diagnostics(
        &self,
        files: &SortedMap<FileId, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<Vec<String>>;
}
