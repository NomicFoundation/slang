mod slang;
mod solc;

use std::collections::BTreeMap;

use anyhow::Result;
use semver::Version;
pub(crate) use slang::SlangTarget;
pub(crate) use solc::SolcTarget;

pub(crate) trait TestTarget {
    fn name(&self) -> &'static str;

    fn collect_diagnostics(
        &self,
        files: &BTreeMap<String, String>,
        version: &Version,
    ) -> Result<Vec<String>>;
}
