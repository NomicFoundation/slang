//! Compiles a fixed set of in-memory sources, shared by the v2 test harnesses.

use slang_solidity_v2::compilation::{
    CompilationBuilder, CompilationBuilderConfig, CompilationUnit,
};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::path_resolver;

/// Compiles `files` at the given language version and EVM target.
///
/// Every source is added as a root, so a source that is not imported gets
/// analyzed too.
pub fn compile(
    files: &SortedMap<FileId, String>,
    version: LanguageVersion,
    target: EvmTarget,
) -> CompilationUnit {
    let mut builder = CompilationBuilder::create(
        version,
        target,
        InMemoryConfig {
            files: files.clone(),
        },
    );

    for file_id in files.keys() {
        builder.add_file(file_id.clone());
    }

    builder.build()
}

/// Serves the sources it was given; anything else is reported as a missing file
/// rather than a harness error.
struct InMemoryConfig {
    files: SortedMap<FileId, String>,
}

impl CompilationBuilderConfig for InMemoryConfig {
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
        Ok(path_resolver::resolve_import(source_file_id, import_path))
    }
}
