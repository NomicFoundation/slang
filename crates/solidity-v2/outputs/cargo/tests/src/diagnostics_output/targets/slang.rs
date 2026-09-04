use anyhow::Result;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_testing_utils::compilation;
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::diagnostics_output::targets::{TargetOutcome, TestTarget};
use crate::snapshots::SnapshotStatus;

pub(crate) struct SlangTarget;

impl TestTarget for SlangTarget {
    fn name(&self) -> &'static str {
        "slang"
    }

    fn compile(
        &self,
        files: &SortedMap<FileId, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<TargetOutcome> {
        let compilation = compilation::compile(files, version, evm_target);

        let status = SnapshotStatus::from_diagnostics(compilation.diagnostics());

        let rendered = compilation
            .diagnostics()
            .iter()
            .map(|diagnostic| {
                let file_id = diagnostic.file_id();
                let source = files.get(file_id).cloned().unwrap_or_default();

                diagnostic::render(diagnostic, file_id.as_str(), &source, false)
            })
            .collect();

        Ok(TargetOutcome {
            diagnostics: rendered,
            status,
        })
    }
}
