#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
//! Slang is intended to be a modular Solidity compiler, specifically targeting code analysis and developer tooling.
//! This means servicing tools with domain-specific APIs and, in general, facilitating working with and analyzing the
//! Solidity source code. If you're in the editor writing Solidity or performing linting or additional validation,
//! there's a chance that you are, or could be, running Slang!
//!
//! # Parsing Solidity Source Code
//!
//! ```
//! use slang_solidity::parser::Parser;
//! use slang_solidity::LanguageFacts;
//!
//! let source = "contract Foo {}";
//! let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
//! let parse_output = parser.parse_file_contents(&source);
//! ```
//! # Using the Cursor: Listing Contract Names
//!
//! [`Cursors`][`crate::cst::Cursor`] are the simplest way to navigate a CST. The below example shows how you might
//! use cursors to list all of the contract names in a source file.
//! ```
//! // Step 1 (not shown): get a parse tree
//! // Step 2: Get a cursor
//! let mut cursor = tree.create_tree_cursor();
//!
//! // Step 3: Use the cursor to navigate to all `ContractDefinition`'s and print out their names
//! while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition) {
//!     println!("Contract: {}", cursor.node.unparse());
//! }
//! ```

mod extensions;
mod generated;

pub use generated::*;

#[cfg(feature = "__private_backend_api")]
pub mod backend;
