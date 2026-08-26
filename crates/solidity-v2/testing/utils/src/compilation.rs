//! Compiles a fixed set of in-memory sources, shared by the v2 test harnesses.

use slang_solidity_v2::compilation::{CompilationUnit, ImportResolver};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;
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
    CompilationUnit::create(version, target, files.clone(), InMemoryResolver)
}

struct InMemoryResolver;

impl ImportResolver for InMemoryResolver {
    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(path_resolver::resolve_import(source_file_id, import_path))
    }
}
