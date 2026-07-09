use anyhow::Result;
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig, FileId};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::diagnostics_output::targets::TestTarget;
use crate::utils::path_resolver;

pub(crate) struct SlangTarget;

impl TestTarget for SlangTarget {
    fn name(&self) -> &'static str {
        "slang"
    }

    fn collect_diagnostics(
        &self,
        files: &SortedMap<FileId, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<Vec<String>> {
        let config = TestConfig {
            files: files.clone(),
        };
        let mut builder = CompilationBuilder::create(version, evm_target, config);

        for file in files.keys() {
            builder.add_file(file.clone());
        }

        let compilation = builder.build();
        let diagnostics = compilation.diagnostics();

        let mut rendered = Vec::new();
        for diagnostic in diagnostics {
            let file_id = diagnostic.file_id();
            let source = files.get(file_id).cloned().unwrap_or_default();
            rendered.push(diagnostic::render(
                diagnostic,
                file_id.as_str(),
                &source,
                false,
            ));
        }

        Ok(rendered)
    }
}

struct TestConfig {
    files: SortedMap<FileId, String>,
}

impl CompilationBuilderConfig for TestConfig {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        self.files.get(file_id).cloned().ok_or_else(|| MissingFile {
            reason: "File not found.".to_string(),
        })
    }

    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        path_resolver::resolve_import(source_file_id, import_path).ok_or_else(|| UnresolvedImport {
            reason: "Unresolved import.".to_string(),
        })
    }
}
