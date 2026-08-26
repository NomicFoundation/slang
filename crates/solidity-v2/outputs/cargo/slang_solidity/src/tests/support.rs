//! Helpers shared by the sibling test modules.

use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationUnit, Configuration, FileId, ImportResolver};
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::utils::LanguageVersion;

/// Resolves every import path to a file of the same name.
pub(super) struct TestImportResolver;

impl ImportResolver for TestImportResolver {
    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(import_path.into())
    }
}

/// Compiles the given sources at the latest language version and EVM target,
/// resolving imports with [`TestImportResolver`].
pub(super) fn compile<'s>(sources: impl IntoIterator<Item = (FileId, &'s str)>) -> CompilationUnit {
    CompilationUnit::create(Configuration {
        language_version: LanguageVersion::LATEST,
        evm_target: EvmTarget::LATEST,
        sources,
        resolver: TestImportResolver,
    })
}

/// Renders a minimal contract of the given name that imports the given paths.
pub(super) fn contract(name: &str, imports: &[&str]) -> String {
    use std::fmt::Write;

    let imports = imports.iter().fold(String::new(), |mut text, path| {
        writeln!(text, "import \"{path}\";").unwrap();
        text
    });

    format!("pragma solidity ^0.8.0;\n{imports}\ncontract {name} {{}}\n")
}
