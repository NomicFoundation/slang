use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;

use semver::Version;

use crate::compilation::internal_builder::{CompilationInitializationError, ResolveImportError};
use crate::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};
use crate::cst::Cursor;

/// User-provided options and callbacks necessary for the `CompilationBuilder` to perform its job.
pub trait CompilationBuilderConfig<E> {
    /// Callback used by this builder to load the contents of a file.
    ///
    /// The user is responsible for fetching the file from the filesystem.
    /// If the file is not found, the callback should return `None`.
    /// Any errors returned by the callback will be propagated to the caller.
    fn read_file(&self, file_id: &str) -> Result<Option<String>, E>;

    /// Callback used by this builder to resolve an import path.
    /// For example, if a source file contains the following statement:
    ///
    /// ```solidity
    /// import {Foo} from "foo.sol";
    /// ```
    ///
    /// Then the API will invoke the callback with a cursor pointing to the `"foo.sol"` string literal.
    ///
    /// The user is responsible for resolving it to a file in the compilation, and return its ID.
    /// If the callback returns `None`, the import will stay unresolved.
    /// Any errors returned by the callback will be propagated to the caller.
    fn resolve_import(
        &self,
        source_file_id: &str,
        import_path: &Cursor,
    ) -> Result<Option<String>, E>;
}

/// A builder for creating compilation units.
/// Allows incrementally building a list of all files and their imports.
pub struct CompilationBuilder<E> {
    /// The user-supplied configuration.
    pub config: Rc<dyn CompilationBuilderConfig<E>>,

    internal: InternalCompilationBuilder,
    seen_files: HashSet<String>,
}

/// Possible errors for `CompilationBuilder::add_file`.
pub enum AddFileError<E> {
    ResolveImportError(ResolveImportError),
    UserError(E),
}

impl<E> CompilationBuilder<E> {
    /// Creates a new compilation builder for the specified language version.
    pub fn new(
        version: Version,
        config: Rc<dyn CompilationBuilderConfig<E>>,
    ) -> Result<CompilationBuilder<E>, CompilationInitializationError> {
        Ok(CompilationBuilder {
            config,
            internal: InternalCompilationBuilder::create(version)?,
            seen_files: HashSet::new(),
        })
    }

    /// Adds a source file to the compilation unit.
    /// Typically, users only need to add the "root" file, which contains the main contract they are trying to analyze.
    /// Any files that are imported by the root file will be discovered and loaded automatically by the config callbacks.
    ///
    /// Adding multiple files (roots) is supported. For example, an IDE can choose to add all NPM dependencies,
    /// regardless of whether they are imported or not, to be able to query the definitions there.
    ///
    /// Adding a file that has already been added is a no-op.
    pub fn add_file(&mut self, filename: &str) -> Result<(), AddFileError<E>> {
        if !self.seen_files.insert(filename.into()) {
            return Ok(());
        }

        let source = self
            .config
            .read_file(filename)
            .map_err(|e| AddFileError::UserError(e))?;

        if let Some(source) = source {
            let AddFileResponse { import_paths } = self.internal.add_file(filename.into(), &source);

            for import_path_cursor in import_paths {
                let import_id = self
                    .config
                    .resolve_import(filename, &import_path_cursor)
                    .map_err(|e| AddFileError::UserError(e))?;

                if let Some(import_id) = &import_id {
                    self.internal
                        .resolve_import(filename, &import_path_cursor, import_id.clone())
                        .map_err(|e| AddFileError::ResolveImportError(e))?;
                    self.add_file(import_id)?;
                }
            }
        }
        Ok(())
    }

    /// Builds and returns the final compilation unit.
    pub fn build(&self) -> CompilationUnit {
        self.internal.build()
    }
}
