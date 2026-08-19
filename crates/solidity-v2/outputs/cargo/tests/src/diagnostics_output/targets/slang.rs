use anyhow::Result;
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig, FileId};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::diagnostics_output::targets::{TargetOutcome, TestTarget};
use crate::utils::path_resolver;

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
        let mut builder = CompilationBuilder::create(version, evm_target, TestConfig);
        builder.add_files(files.clone());

        let compilation = builder.build();

        let diagnostics: Vec<_> = compilation.diagnostics().iter().collect();

        let compilation_succeeded = !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error);

        let rendered = diagnostics
            .into_iter()
            .map(|diagnostic| {
                let file_id = diagnostic.file_id();
                let source = files.get(file_id).cloned().unwrap_or_default();

                diagnostic::render(diagnostic, file_id.as_str(), &source, false)
            })
            .collect();

        Ok(TargetOutcome {
            diagnostics: rendered,
            compilation_succeeded,
        })
    }
}

struct TestConfig;

impl CompilationBuilderConfig for TestConfig {
    fn resolve_import(
        &self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        path_resolver::resolve_import(source_file_id, import_path).ok_or_else(|| UnresolvedImport {
            reason: "Unresolved import.".to_string(),
        })
    }
}
