use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, SemanticContext, SemanticFile,
};

use super::file::File;
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
    /// Then the API will invoke the callback with the value of the `"foo.sol"`
    /// string literal, including the surrounding quotes.
    ///
    /// The user is responsible for resolving it to a file in the compilation,
    /// and returning its ID. Any error returned is surfaced as a compilation
    /// diagnostic on the [`CompilationUnit`].
    fn resolve_import(&mut self, source_file_id: &str, import_path: &str)
        -> Result<String, String>;
}

/// A builder for creating compilation units.
/// Allows incrementally building a list of all files and their imports.
pub struct CompilationBuilder<C: CompilationBuilderConfig> {
    /// The user-supplied configuration.
    pub config: C,

    language_version: LanguageVersion,
    diagnostics: DiagnosticCollection,

    files: BTreeMap<String, File>,
    seen_files: HashSet<String>,
}

impl<C: CompilationBuilderConfig> CompilationBuilder<C> {
    /// Creates a new compilation builder for the specified language version and callbacks.
    pub fn create(language_version: LanguageVersion, config: C) -> CompilationBuilder<C> {
        CompilationBuilder {
            config,

            language_version,
            diagnostics: DiagnosticCollection::new(),

            files: BTreeMap::new(),
            seen_files: HashSet::new(),
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
                self.diagnostics.push(file_id, 0..0, MissingFile { reason });
                return;
            }
        };

        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse(&file_id, &source, self.language_version);
        self.diagnostics.extend(diagnostics);

        let BuildOutput {
            source_unit: ir_root,
            diagnostics,
        } = ir::build(&file_id, &source_unit, &source);
        self.diagnostics.extend(diagnostics);

        let mut file = File::new(file_id.clone(), ir_root);
        let import_paths = extract_import_paths_from_source_unit(file.ir_root());
        for (node_id, import_path) in import_paths {
            match self.config.resolve_import(&file_id, &import_path) {
                Ok(resolved_id) => {
                    file.add_resolved_import(node_id, resolved_id.clone());
                    self.add_file(resolved_id);
                }
                Err(reason) => {
                    self.diagnostics.push(
                        file_id.clone(),
                        // TODO(v2): surface import path range
                        0..0,
                        UnresolvedImport { reason },
                    );
                }
            }
        }

        self.files.insert(file_id, file);
    }

    /// Builds and returns the final compilation unit.
    pub fn build(self) -> CompilationUnit {
        let files: Vec<File> = self.files.into_values().collect();
        let semantic = SemanticContext::build_from(self.language_version, &files);

        CompilationUnit::create(
            self.language_version,
            files,
            Rc::new(semantic),
            self.diagnostics,
        )
    }
}
