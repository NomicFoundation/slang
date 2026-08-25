use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
    ///
    /// Parsing runs in parallel on [`rayon`]'s global thread pool. The result
    /// does not depend on how large that pool is, so this is only ever a
    /// question of speed; to bound it, call this inside
    /// [`rayon::ThreadPool::install`] on a pool of your own.
    // TODO(wasm): `rayon` falls back to the calling
    // thread for its *implicit* global pool — which is why this already builds
    // for `wasm32-wasip1`, but a pool built
    // explicitly errors instead. Revisit this, and consider gating `rayon`
    // behind a feature so the scheduler stays out of that build, if v2 gains a
    // wasm interface.
    pub fn create<'s, S, R>(config: Configuration<S, R>) -> CompilationUnit
    where
        S: IntoIterator<Item = (FileId, &'s str)>,
        R: ImportResolver,
    {
        let Configuration {
            language_version,
            evm_target,
            sources,
            mut resolver,
        } = config;

        let mut diagnostics = DiagnosticCollection::default();

        let sources = collect_sources(sources.into_iter(), &mut diagnostics);

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
fn collect_sources<'s>(
    sources: impl Iterator<Item = (FileId, &'s str)>,
    diagnostics: &mut DiagnosticCollection,
) -> SortedMap<FileId, &'s str> {
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
struct ParsedFile<'s> {
    file_id: FileId,
    contents: &'s str,
    source_unit: cst::SourceUnit,
}

/// Parses every source file, in parallel over [`rayon`]'s global thread pool.
///
/// The result must stay sorted by [`FileId`], since [`build_ir`] hands out node
/// ids by walking it — hence the `Vec` and the *indexed* `unzip`, which fills
/// the output by input position rather than by whoever finishes first.
///
/// TODO(v2): giving each file an independent node-id space would free this
/// phase to schedule as it likes. It'll be necessary once IR building is parallelized.
fn parse_files<'s>(
    sources: SortedMap<FileId, &'s str>,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) -> Vec<ParsedFile<'s>> {
    let parse =
        |(file_id, contents): (FileId, &'s str)| parse_file(file_id, contents, language_version);

    // Using rayon's parallel iterator has some costs that are
    // not worth it when a single file is being processed.
    let (parsed_files, per_file_diagnostics): (Vec<_>, Vec<_>) = if sources.len() < 2 {
        sources.into_iter().map(parse).unzip()
    } else {
        // Through a `Vec` because that is the only route rayon guarantees as an
        // indexed parallel iterator, which is what fills `unzip`'s output by
        // input position.
        // See the TODO in this function, this requirement should go
        // once order is relaxed.
        Vec::from_iter(sources).into_par_iter().map(parse).unzip()
    };

    for parse_diagnostics in per_file_diagnostics {
        diagnostics.extend(parse_diagnostics);
    }

    parsed_files
}

/// Parses one source file, returning its diagnostics rather than pushing them
/// into a shared collection, so that it can run on any thread.
fn parse_file(
    file_id: FileId,
    contents: &str,
    language_version: LanguageVersion,
) -> (ParsedFile<'_>, DiagnosticCollection) {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&file_id, contents, language_version);

    let parsed_file = ParsedFile {
        file_id,
        contents,
        source_unit,
    };

    (parsed_file, diagnostics)
}

/// Lowers every parsed file into its IR representation, resolving the import
/// paths it declares onto the IR nodes that declare them.
///
/// Because the full set of files is known up front, an import resolving outside
/// of it is reported here, rather than being discovered while loading files.
fn build_ir<R: ImportResolver>(
    resolver: &mut R,
    parsed_files: Vec<ParsedFile<'_>>,
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
