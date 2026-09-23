use std::cmp::Reverse;

use rayon::iter::{ParallelBridge, ParallelIterator};
use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::diagnostics::kinds::compilation::{
    DuplicatedFileId, MissingImportedFile,
};
use slang_solidity_v2_common::diagnostics::{DiagnosticCollection, DiagnosticSeverity};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    SemanticContext, SemanticFile, SourceUnitImport, extract_imports_from_source_unit,
};

use super::configuration::{Configuration, ImportResolver};
use super::file::InternalFile;
use super::unit::CompilationUnit;
use super::validation::validate_cst;

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
    /// Parsing and IR building run in parallel over [`rayon`]'s ambient thread
    /// pool. The result does not depend on how large that pool is, so this is
    /// only ever a question of speed; to bound it, call this inside
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

        let lowered_files = parse_and_build_ir(sources, language_version);
        let (files, node_kinds) =
            merge_lowered_files(&mut resolver, lowered_files, &mut diagnostics);

        let semantic = SemanticContext::build_from(
            language_version,
            evm_target,
            &files,
            Some(&node_kinds),
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

/// One source file, parsed and lowered to the IR.
struct LoweredFile {
    file: InternalFile,
    imports: Vec<SourceUnitImport>,
    node_kinds: ir::NodeKindHistogram,
    diagnostics: DiagnosticCollection,
}

/// Parses and build the IR for every source file, one task per file, in parallel over
/// [`rayon`]'s ambient thread pool.
fn parse_and_build_ir(
    sources: SortedMap<FileId, &str>,
    language_version: LanguageVersion,
) -> Vec<LoweredFile> {
    let parse_and_lower = |((file_id, contents), id_generator)| {
        parse_and_lower_file(file_id, contents, id_generator, language_version)
    };

    let single_file = sources.len() < 2;
    // Each file is paired with its `NodeIdGenerator` before processing starts.
    let paired = sources.into_iter().zip(ir::NodeIdGroups::default());

    // Using rayon's parallel iterator has some costs that are
    // not worth it when a single file is being processed.
    if single_file {
        return paired.map(parse_and_lower).collect();
    }

    // Longest-processing-time-first: sort the files by size, good scheduling
    // heuristic.
    let mut ordered = paired.collect::<Vec<_>>();
    ordered.sort_by_key(|((_, contents), _)| Reverse(contents.len()));

    // `into_par_iter` splits the collection into contiguous chunks, and schedules
    // each chunk with a worker, that means the first chunk will have all the heavy
    // work, and the last chunk all the light work.
    //
    // `par_bridge` is a bit slower, since it synchronizes the threads and gives out
    // tasks one at a time, but it means tasks are scheduled in order (from heaviest
    // to lightest)
    ordered
        .into_iter()
        .par_bridge()
        .map(parse_and_lower)
        .collect()
}

/// Parses one source file, lowers it into its IR representation.
/// It returns it together with its diagnostics, its node kinds histogram,
/// and its unresolved imports.
fn parse_and_lower_file(
    file_id: FileId,
    contents: &str,
    mut id_generator: ir::NodeIdGenerator,
    language_version: LanguageVersion,
) -> LoweredFile {
    let ParseOutput {
        source_unit,
        diagnostics: mut file_diagnostics,
    } = Parser::parse(&file_id, contents, language_version);

    if match file_diagnostics.highest_severity() {
        Some(DiagnosticSeverity::Error) => false,
        Some(DiagnosticSeverity::Warning) => true,
        None => true,
    } {
        // Only run validation if there are no parser errors.
        // Otherwise, we risk validating error-recovery "stub" nodes.
        validate_cst(&source_unit, &file_id, &mut file_diagnostics);
    }

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

    file_diagnostics.extend(ir_diagnostics);

    let file = InternalFile::new(file_id, ir_root);
    let imports = extract_imports_from_source_unit(file.ir_root());

    LoweredFile {
        file,
        imports,
        node_kinds: id_generator.into_histogram(),
        diagnostics: file_diagnostics,
    }
}

/// Merges the per-file results of the parallel stage into the compilation:
/// resolves each file's imports against the full set, and folds the per-file
/// diagnostics and node-kind histograms into the whole-compilation ones.
///
/// Because the full set of files is known up front, an import resolving outside
/// of it is reported here, rather than being discovered while loading files.
///
/// This runs on the calling thread, because [`ImportResolver::resolve_import`]
/// takes `&mut self`. It is sequential but *not* ordered: files arrive in
/// whatever order the pool finished them, so a resolver that keeps state must
/// not read anything into the order it is called in.
fn merge_lowered_files<R: ImportResolver>(
    resolver: &mut R,
    lowered_files: Vec<LoweredFile>,
    diagnostics: &mut DiagnosticCollection,
) -> (Vec<InternalFile>, ir::NodeKindHistogram) {
    // Cloning a `FileId` is only a reference-count bump, so collecting them all
    // up front is cheap, and lets every file be resolved against the full set.
    let known_files: Set<FileId> = lowered_files
        .iter()
        .map(|lowered_file| lowered_file.file.id().clone())
        .collect();

    let mut node_kinds = ir::NodeKindHistogram::default();
    let mut files = Vec::with_capacity(lowered_files.len());

    for lowered_file in lowered_files {
        let LoweredFile {
            mut file,
            imports,
            node_kinds: file_node_kinds,
            diagnostics: file_diagnostics,
        } = lowered_file;

        diagnostics.extend(file_diagnostics);
        node_kinds.absorb(&file_node_kinds);

        for SourceUnitImport {
            node_id,
            path,
            range,
        } in imports
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

        files.push(file);
    }

    (files, node_kinds)
}
