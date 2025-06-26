// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::HashSet;

use semver::Version;
use strum_macros::Display;

use crate::compilation::internal_builder::{
    AddFileResponse, CompilationInitializationError, InternalCompilationBuilder,
};
use crate::compilation::CompilationUnit;
use crate::cst::Cursor;

/// A `CompilationBuilder` is a convenient utility to build a `CompilationUnit` for a multi-file project.
/// In order to do its job, the user must provide the necessary callbacks to obtain each file.
/// Example:
///
/// ```
/// struct MyProjectConfig {
/// }
///
/// impl CompilationBuilderConfig<String> for MyProjectConfig {
///   fn read_file(&self, file_id: &str) -> std::result::Result<Option<String>, String> {
///       match file_id {
///         "b.sol" => Ok(Some("require 'a.sol'; contract B is A { }".into())),
///         "a.sol" => Ok(Some("contract A { }".into())),
///         _ => Err(format!("Unknown file: {file_id}"))
///       }
///   }
///
///   fn resolve_import(
///       &self,
///       source_file_id: &str,
///       import_path_cursor: &slang_solidity::cst::Cursor,
///   ) -> std::result::Result<Option<String>, String> {
///       let import_path = import_path_cursor.node().unparse();
///       let import_path = import_path[1..-1]; // strip off the quotes
///
///       Ok(import_path)
///   }
/// }
///
/// # use slang_solidity::utils::LanguageFacts;
/// let config = MyProjectConfig { };
/// let builder = CompilationBuilder::new(LanguageFacts::LATEST_VERSION, &config);
/// builder.add_file("b.sol")?;
/// let unit = builder.build();
/// assert_eq!(unit.files().len(), 2);
/// ```

/// User-provided callbacks necessary for the `CompilationBuilder` to perform its job.
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
        import_path_cursor: &Cursor,
    ) -> Result<Option<String>, E>;
}

/// A builder for creating compilation units.
/// Allows incrementally building a list of all files and their imports.
pub struct CompilationBuilder<'b, E> {
    /// The user-supplied configuration.
    pub config: &'b dyn CompilationBuilderConfig<E>,

    internal: InternalCompilationBuilder,
    seen_files: HashSet<String>,
}

impl<'b, E> CompilationBuilder<'b, E> {
    /// Creates a new compilation builder for the specified language version and callbacks.
    pub fn new(
        version: Version,
        config: &'b dyn CompilationBuilderConfig<E>,
    ) -> Result<CompilationBuilder<'b, E>, CompilationInitializationError> {
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
    pub fn add_file(&mut self, filename: &str) -> Result<(), E> {
        if !self.seen_files.insert(filename.into()) {
            return Ok(());
        }

        let source = self.config.read_file(filename)?;

        if let Some(source) = source {
            let AddFileResponse { import_paths } = self.internal.add_file(filename.into(), &source);

            for import_path_cursor in import_paths {
                let import_id = self.config.resolve_import(filename, &import_path_cursor)?;

                if let Some(import_id) = &import_id {
                    self.internal
                        .resolve_import(filename, &import_path_cursor, import_id.clone())
                        .unwrap_or_else(|_| panic!("{filename} should have been added"));
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
