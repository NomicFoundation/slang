use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{
    DuplicatedFileId, MissingImportedFile,
};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as cst;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    SemanticContext, SemanticFile, SourceUnitImport, extract_imports_from_source_unit,
};

use super::configuration::{Configuration, ImportResolver};
use super::file::InternalFile;
use super::unit::CompilationUnit;

impl CompilationUnit {
    /// Compiles the given source files into a [`CompilationUnit`].
    ///
    /// The caller is responsible for providing every file that takes part in
    /// the compilation, including the transitive imports of the files they care
    /// about. An import that resolves to a file which was not provided is
    /// reported as a [`MissingImportedFile`] diagnostic on the returned unit.
    ///
    /// Providing the same file ID more than once keeps the last contents given
    /// for it, and reports a [`DuplicatedFileId`] diagnostic for each
    /// repetition.
    ///
    /// All of the work — parsing, IR building, semantic analysis — happens
    /// here, and every problem it runs into is reported as a diagnostic on the
    /// returned unit. Parse errors, unresolvable imports, and missing imported
    /// files are all collected this way — see [`CompilationUnit::diagnostics`].
    pub fn create<S, R>(config: Configuration<S, R>) -> CompilationUnit
    where
        S: IntoIterator<Item = (FileId, String)>,
        R: ImportResolver,
    {
        let Configuration {
            language_version,
            evm_target,
            sources,
            mut resolver,
        } = config;

        let mut diagnostics = DiagnosticCollection::default();

        let sources = collect_sources(sources, &mut diagnostics);

        let parsed_files = parse_files(sources, language_version, &mut diagnostics);
        let (files, id_generator) = build_ir(
            &mut resolver,
            parsed_files,
            language_version,
            &mut diagnostics,
        );

        let semantic = SemanticContext::build_from(
            language_version,
            evm_target,
            &files,
            Some(id_generator.histogram()),
            &mut diagnostics,
        );

        CompilationUnit::from_parts(language_version, evm_target, files, semantic, diagnostics)
    }
}

/// Collects the given sources by file ID, keeping the last contents given for
/// an ID and reporting every repetition as a [`DuplicatedFileId`] diagnostic.
fn collect_sources(
    sources: impl IntoIterator<Item = (FileId, String)>,
    diagnostics: &mut DiagnosticCollection,
) -> SortedMap<FileId, String> {
    let mut collected = SortedMap::default();

    for (file_id, contents) in sources {
        if collected.insert(file_id.clone(), contents).is_some() {
            // TODO(v2): We should consider a proper way to report diagnostics
            // that don't belong to a specific range, or even a specific file.
            // For now, we report it at the start of the file.
            diagnostics.push(file_id.clone(), 0..0, DuplicatedFileId { file_id });
        }
    }

    collected
}

/// One source file, parsed.
struct ParsedFile {
    file_id: FileId,
    contents: String,
    source_unit: cst::SourceUnit,
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
fn build_ir<R: ImportResolver>(
    resolver: &mut R,
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
            for SourceUnitImport {
                node_id,
                path,
                range,
            } in extract_imports_from_source_unit(file.ir_root())
            {
                let imported_file_id = match resolver.resolve_import(file.id(), &path) {
                    Ok(imported_file_id) => imported_file_id,
                    Err(unresolved_import) => {
                        diagnostics.push(file.id().clone(), range, unresolved_import);
                        continue;
                    }
                };

                if !known_files.contains(&imported_file_id) {
                    diagnostics.push(
                        file.id().clone(),
                        range,
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
