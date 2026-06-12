mod slang;
mod solc;

use anyhow::Result;
use semver::Version;
pub(crate) use slang::SlangTarget;
use slang_solidity_v2_common::collections::SortedMap;
pub(crate) use solc::SolcTarget;

pub(crate) trait TestTarget {
    fn name(&self) -> &'static str;

    fn collect_diagnostics(
        &self,
        files: &SortedMap<String, String>,
        version: &Version,
    ) -> Result<Vec<String>>;
}
