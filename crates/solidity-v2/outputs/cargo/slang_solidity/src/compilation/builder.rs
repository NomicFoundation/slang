use slang_solidity_v2_common::collections::{Set, SortedMap, SortedSet};
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::utils::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as cst;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, SemanticContext, SemanticFile,
};

use super::file::InternalFile;
use super::unit::CompilationUnit;

/// User-provided callbacks necessary for the `CompilationBuilder` to perform its job.
pub trait CompilationBuilderConfig {
    /// Callback used by this builder to load the contents of a file.
    ///
    /// The user is responsible for fetching the file from the filesystem. Any
    /// error returned is surfaced as a compilation diagnostic on the [`CompilationUnit`].
    fn read_file(&mut self, file_id: &str) -> Result<String, String>;

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
    /// and returning its ID. Any error returned is surfaced as a compilation
    /// diagnostic on the [`CompilationUnit`].
    fn resolve_import(&mut self, source_file_id: &str, import_path: &str)
        -> Result<String, String>;
}

struct BuilderFile {
    source_unit: cst::SourceUnit,
    contents: String,
    resolved_imports: SortedMap<String, String>,
}

/// A builder for creating compilation units.
/// Allows incrementally building a list of all files and their imports.
pub struct CompilationBuilder<C: CompilationBuilderConfig> {
    /// The user-supplied configuration.
    pub config: C,

    language_version: LanguageVersion,
    diagnostics: DiagnosticCollection,

    files: SortedMap<String, BuilderFile>,
    seen_files: Set<String>,
}

impl<C: CompilationBuilderConfig> CompilationBuilder<C> {
    /// Creates a new compilation builder for the specified language version and callbacks.
    pub fn create(language_version: LanguageVersion, config: C) -> CompilationBuilder<C> {
        CompilationBuilder {
            config,

            language_version,
            diagnostics: DiagnosticCollection::default(),

            files: SortedMap::new(),
            seen_files: Set::default(),
        }
    }

    /// Adds a source file to the compilation unit. Typically, users only need
    /// to add the "root" file, which contains the main contract they are trying
    /// to analyze. Any files that are imported by the root file will be
    /// discovered and loaded automatically by the config callbacks.
    ///
    /// Adding multiple files (roots) is supported. For example, an IDE can
    /// choose to add all NPM dependencies, regardless of whether they are
    /// imported or not, to be able to query the definitions there.
    ///
    /// Adding a file that has already been added is a no-op.
    ///
    /// Parse errors, unresolvable imports, and missing files are collected as
    /// diagnostics on the resulting [`CompilationUnit`] — see
    /// [`CompilationUnit::diagnostics`].
    pub fn add_file(&mut self, file_id: String) {
        if !self.seen_files.insert(file_id.clone()) {
            return;
        }

        let source = match self.config.read_file(&file_id) {
            Ok(source) => source,
            Err(reason) => {
                // TODO(validation) SDR[1742]: instead of `file_id`, these errors should be placed
                // on all imports that need it, with the import path text range.
                self.diagnostics.push(file_id, 0..0, MissingFile { reason });
                return;
            }
        };

        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse(&file_id, &source, self.language_version);
        self.diagnostics.extend(diagnostics);

        let import_paths = extract_import_paths_from_cst(&source_unit, &source);

        let mut resolved_imports = SortedMap::new();
        for import_path in import_paths {
            match self.config.resolve_import(&file_id, &import_path) {
                Ok(resolved_id) => {
                    resolved_imports.insert(import_path, resolved_id.clone());
                    self.add_file(resolved_id);
                }
                Err(reason) => {
                    self.diagnostics.push(
                        file_id.clone(),
                        // TODO(validation) SDR[1743]: surface import path range
                        0..0,
                        UnresolvedImport { reason },
                    );
                }
            }
        }

        self.files.insert(
            file_id,
            BuilderFile {
                source_unit,
                contents: source,
                resolved_imports,
            },
        );
    }

    /// Builds and returns the final compilation unit.
    pub fn build(self) -> CompilationUnit {
        let language_version = self.language_version;
        let mut diagnostics = self.diagnostics;

        let mut id_generator = ir::NodeIdGenerator::default();
        let files: Vec<InternalFile> = self
            .files
            .into_iter()
            .map(|(file_id, internal_file)| {
                let BuildOutput {
                    ir_root,
                    diagnostics: ir_diagnostics,
                } = ir::build(
                    &file_id,
                    &internal_file.source_unit,
                    &internal_file.contents,
                    &mut id_generator,
                );
                diagnostics.extend(ir_diagnostics);

                let mut file = InternalFile::new(file_id, ir_root);
                for (node_id, import_path) in extract_import_paths_from_source_unit(file.ir_root())
                {
                    if let Some(target_file_id) = internal_file.resolved_imports.get(&import_path) {
                        file.add_resolved_import(node_id, target_file_id.clone());
                    }
                }
                file
            })
            .collect();

        let semantic = SemanticContext::build_from(language_version, &files, &mut diagnostics);

        CompilationUnit::create(language_version, files, semantic, diagnostics)
    }
}

fn extract_import_paths_from_cst(
    source_unit: &cst::SourceUnit,
    contents: &str,
) -> SortedSet<String> {
    let mut import_paths = SortedSet::new();

    for member in &source_unit.members.elements {
        let cst::SourceUnitMember::ImportDirective(import_directive) = member else {
            continue;
        };
        let range = match &import_directive.clause {
            cst::ImportClause::PathImport(path_import) => &path_import.path.range,
            cst::ImportClause::NamedImport(named_import) => &named_import.path.range,
            cst::ImportClause::ImportDeconstruction(import_deconstruction) => {
                &import_deconstruction.path.range
            }
        };
        let literal = &contents[range.clone()];
        import_paths.insert(strip_string_literal_quotes(literal).to_owned());
    }
    import_paths
}
