use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{
    MissingImportedFile, UnresolvedImport,
};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as cst;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    SemanticContext, SemanticFile, extract_import_paths_from_source_unit,
};

use super::file::InternalFile;
use super::unit::CompilationUnit;

/// User-provided callbacks necessary for the `CompilationBuilder` to perform its job.
pub trait CompilationBuilderConfig {
    /// Callback used by this builder to resolve an import path.
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
    /// compilation diagnostic on the [`CompilationUnit`].
    ///
    /// Resolving to a file that was never added to the builder yields a
    /// [`MissingImportedFile`] diagnostic instead.
    fn resolve_import(
        &self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport>;
}

/// A builder for creating compilation units.
///
/// Collects the source files that make up a compilation, then turns them into a
/// [`CompilationUnit`] in [`build()`](CompilationBuilder::build). Adding files
/// only records them; all of the work (parsing, IR building, semantic analysis)
/// happens in `build()`, and every problem it runs into is reported as a
/// diagnostic on the resulting unit.
pub struct CompilationBuilder<C: CompilationBuilderConfig> {
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    config: C,

    sources: SortedMap<FileId, String>,
}

/// One source file, parsed.
struct ParsedFile {
    file_id: FileId,
    contents: String,
    source_unit: cst::SourceUnit,
}

impl<C: CompilationBuilderConfig> CompilationBuilder<C> {
    /// Creates a new compilation builder for the specified language version,
    /// EVM target, and resolver callbacks.
    pub fn create(
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        config: C,
    ) -> CompilationBuilder<C> {
        CompilationBuilder {
            language_version,
            evm_target,
            config,

            sources: SortedMap::default(),
        }
    }

    /// Adds a source file, and its contents, to the compilation unit.
    ///
    /// The user is responsible for providing every file that takes part in the
    /// compilation, including the transitive imports of the files they care
    /// about. An import that resolves to a file which was not added is reported
    /// as a [`MissingImportedFile`] diagnostic on the resulting unit.
    ///
    /// Adding a file that has already been added replaces its contents.
    pub fn add_file(&mut self, file_id: FileId, contents: String) {
        self.sources.insert(file_id, contents);
    }

    /// Adds several source files at once. Equivalent to calling
    /// [`add_file()`](CompilationBuilder::add_file) on each of them.
    pub fn add_files(&mut self, files: impl IntoIterator<Item = (FileId, String)>) {
        self.sources.extend(files);
    }

    /// Consumes the source files added so far, and returns the final
    /// compilation unit.
    ///
    /// Parse errors, unresolvable imports, and missing imported files are all
    /// collected as diagnostics on the returned [`CompilationUnit`] — see
    /// [`CompilationUnit::diagnostics`].
    pub fn build(self) -> CompilationUnit {
        let CompilationBuilder {
            language_version,
            evm_target,
            config,

            sources,
        } = self;

        let mut diagnostics = DiagnosticCollection::default();

        let parsed_files = parse_files(sources, language_version, &mut diagnostics);
        let (files, id_generator) =
            build_ir(&config, parsed_files, language_version, &mut diagnostics);

        let semantic = SemanticContext::build_from(
            language_version,
            evm_target,
            &files,
            Some(id_generator.histogram()),
            &mut diagnostics,
        );

        CompilationUnit::create(language_version, evm_target, files, semantic, diagnostics)
    }
}

/// Parses every source file.
fn parse_files(
    sources: SortedMap<FileId, String>,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) -> Vec<ParsedFile> {
    sources
        .into_iter()
        .map(|(file_id, contents)| {
            let ParseOutput {
                source_unit,
                diagnostics: parse_diagnostics,
            } = Parser::parse(&file_id, &contents, language_version);
            diagnostics.extend(parse_diagnostics);

            ParsedFile {
                file_id,
                contents,
                source_unit,
            }
        })
        .collect()
}

/// Lowers every parsed file into its IR representation, resolving the import
/// paths it declares onto the IR nodes that declare them.
///
/// Because the full set of files is known up front, an import resolving outside
/// of it is reported here, rather than being discovered while loading files.
fn build_ir<C: CompilationBuilderConfig>(
    config: &C,
    parsed_files: Vec<ParsedFile>,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) -> (Vec<InternalFile>, ir::NodeIdGenerator) {
    // Cloning a `FileId` is only a reference-count bump, so collecting them all
    // up front is cheap, and lets every file be resolved against the full set.
    let known_files: Set<FileId> = parsed_files
        .iter()
        .map(|parsed_file| parsed_file.file_id.clone())
        .collect();

    let mut id_generator = ir::NodeIdGenerator::default();

    let files = parsed_files
        .into_iter()
        .map(|parsed_file| {
            let ParsedFile {
                file_id,
                contents,
                source_unit,
            } = parsed_file;

            let BuildOutput {
                ir_root,
                diagnostics: ir_diagnostics,
            } = ir::build(
                &file_id,
                &source_unit,
                &contents,
                language_version,
                &mut id_generator,
            );
            diagnostics.extend(ir_diagnostics);

            let mut file = InternalFile::new(file_id, ir_root);
            for (node_id, import_path, path_range) in
                extract_import_paths_from_source_unit(file.ir_root())
            {
                let imported_file_id = match config.resolve_import(file.id(), &import_path) {
                    Ok(imported_file_id) => imported_file_id,
                    Err(unresolved_import) => {
                        diagnostics.push(file.id().clone(), path_range, unresolved_import);
                        continue;
                    }
                };

                if !known_files.contains(&imported_file_id) {
                    diagnostics.push(
                        file.id().clone(),
                        path_range,
                        MissingImportedFile {
                            imported_file_id: imported_file_id.clone(),
                        },
                    );
                }

                // Recorded even when the file is missing: the diagnostic above
                // is what reports the problem, and the later stages are able to
                // see that the target is not part of the compilation.
                file.add_resolved_import(node_id, imported_file_id);
            }
            file
        })
        .collect();

    (files, id_generator)
}
