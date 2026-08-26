use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;

/// Configuration for creating a compilation unit.
pub struct Configuration<S, R> {
    /// The Solidity language version to use
    pub language_version: LanguageVersion,
    /// The EVM target to compile for
    pub evm_target: EvmTarget,
    /// The source files to compile
    pub sources: S,
    /// The import resolver to use
    pub resolver: R,
}

/// User-provided callback necessary for [`CompilationUnit::create`](super::CompilationUnit::create)
/// to perform its job.
pub trait ImportResolver {
    /// Callback used by the compilation to resolve an import path.
    /// For example, if a source file contains the following statement:
    ///
    /// ```solidity
    /// import {Foo} from "foo.sol";
    /// ```
    ///
    /// Then the API will invoke the callback with the value `foo.sol` (the
    /// contents of the string literal, with the surrounding quotes stripped).
    ///
    /// The user is responsible for resolving it to a file in the compilation,
    /// and returning its ID. The returned [`UnresolvedImport`] is surfaced as a
    /// compilation diagnostic on the [`CompilationUnit`](super::CompilationUnit).
    ///
    /// Resolving to a file that is not part of the compilation yields a
    /// [`MissingImportedFile`](crate::diagnostics::kinds::compilation::MissingImportedFile)
    /// diagnostic instead.
    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport>;
}
