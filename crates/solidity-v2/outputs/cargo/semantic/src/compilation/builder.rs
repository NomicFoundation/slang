use std::collections::HashSet;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::ParserError;

use super::unit::CompilationUnit;
use crate::compilation::internal_builder::{AddFileResponse, InternalCompilationBuilder};

pub enum CompilationBuilderError<E> {
    ParserError(ParserError),
    UserError(E),
}

/// User-provided callbacks necessary for the `CompilationBuilder` to perform its job.
pub trait CompilationBuilderConfig {
    /// User-configurable error type.
    type Error;

    /// Callback used by this builder to load the contents of a file.
    ///
    /// The user is responsible for fetching the file from the filesystem. If
    /// the file is not found, the callback should return `None`. Any errors
    /// returned by the callback will be propagated to the caller.
    fn read_file(&mut self, file_id: &str) -> Result<Option<String>, Self::Error>;

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
    /// and return its ID. If the callback returns `None`, the import will stay
    /// unresolved. Any errors returned by the callback will be propagated to
    /// the caller.
    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path: &str,
    ) -> Result<Option<String>, Self::Error>;
}

/// A builder for creating compilation units.
/// Allows incrementally building a list of all files and their imports.
pub struct CompilationBuilder<E, C: CompilationBuilderConfig<Error = E>> {
    /// The user-supplied configuration.
    pub config: C,

    internal: InternalCompilationBuilder,
    seen_files: HashSet<String>,
}

impl<E, C: CompilationBuilderConfig<Error = E>> CompilationBuilder<E, C> {
    /// Creates a new compilation builder for the specified language version and callbacks.
    pub fn create(version: LanguageVersion, config: C) -> CompilationBuilder<E, C> {
        let internal = InternalCompilationBuilder::create(version);
        CompilationBuilder {
            config,
            internal,
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
    pub fn add_file(&mut self, file_id: &str) -> Result<(), CompilationBuilderError<E>> {
        if !self.seen_files.insert(file_id.into()) {
            return Ok(());
        }

        let source = self
            .config
            .read_file(file_id)
            .map_err(|err| CompilationBuilderError::UserError(err))?;

        if let Some(source) = source {
            let AddFileResponse { import_paths } = self
                .internal
                .add_file(file_id.into(), &source)
                .map_err(|err| CompilationBuilderError::ParserError(err))?;

            for (node_id, import_path) in import_paths {
                let import_id = self
                    .config
                    .resolve_import(file_id, &import_path)
                    .map_err(|err| CompilationBuilderError::UserError(err))?;

                if let Some(import_id) = &import_id {
                    self.internal
                        .resolve_import(file_id, node_id, import_id.clone())
                        .unwrap_or_else(|_| panic!("{file_id} should have been added"));
                    self.add_file(import_id)?;
                }
            }
        }
        Ok(())
    }

    /// Builds and returns the final compilation unit.
    pub fn build(self) -> CompilationUnit {
        self.internal.build()
    }
}
