// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! The `compilation` module provides some basic utilities that users can use to combine multiple source files
//! into a single compilation unit. The compilation unit can then be used to get a complete [`BindingGraph`][`crate::bindings::BindingGraph`]
//! that can resolve references and defintions across all the added files.
//!
//! The main module is a `CompilationBuilder`, a convenient utility to build a `CompilationUnit` for a multi-file project.
//! File names are treated internally as ids: they can refer to the full path of the file, a hash, or any other
//! identification. The user of the builder is responsible to make the connection between imports and file ids.
//!
//! Example:
//!
//! ```
//! use slang_solidity::utils::LanguageFacts;
//! use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig};
//!
//! #[derive(Default)]
//! struct MyProjectConfig {
//! }
//!
//! impl CompilationBuilderConfig for MyProjectConfig {
//!   type Error = String;
//!
//!   fn read_file(&mut self, file_id: &str) -> Result<Option<String>, Self::Error> {
//!       match file_id {
//!         // Loading these files successfully from memory or file system:
//!         "b.sol" => Ok(Some("import 'a.sol'; contract B is A { }".into())),
//!         "a.sol" => Ok(Some("import 'c.sol'; contract A { }".into())),
//!         // If a file is not found, Slang can still partially compile the rest of the files:
//!         "c.sol" => Ok(None),
//!         // Reporting custom errors to caller:
//!         _ => Err(format!("Unknown file: {file_id}"))
//!       }
//!   }
//!
//!   fn resolve_import(
//!       &mut self,
//!       source_file_id: &str,
//!       import_path_cursor: &slang_solidity::cst::Cursor,
//!   ) -> Result<Option<String>, Self::Error> {
//!       let import_path = import_path_cursor.node().unparse();
//!       let import_path = &import_path[1..import_path.len()-1]; // strip off the quotes
//!       Ok(Some(import_path.to_owned())) // as the id, we return the import name as is
//!   }
//! }
//!
//! let config = MyProjectConfig::default();
//! let mut builder = CompilationBuilder::new(LanguageFacts::LATEST_VERSION, config).unwrap();
//! builder.add_file("b.sol").unwrap();
//! let unit = builder.build();
//!
//! // Sanity check: two files were loaded, and there are no parsing errors.
//! assert_eq!(unit.files().len(), 2);
//! assert_eq!(unit.files()[0].errors(), &vec![]);
//! assert_eq!(unit.files()[1].errors(), &vec![]);
//! ```

mod builder;
mod file;
mod internal_builder;
mod unit;

pub use builder::{CompilationBuilder, CompilationBuilderConfig, CompilationInitializationError};
pub use file::File;
#[cfg(feature = "__private_wasm_apis")]
pub use internal_builder::{AddFileResponse, InternalCompilationBuilder};
pub use unit::CompilationUnit;
