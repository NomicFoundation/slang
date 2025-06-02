#![cfg_attr(not(any(feature = "__private_testing_utils", feature = "__private_compilation_api")), deny(missing_docs))]
#![deny(rustdoc::broken_intra_doc_links)]
//! Slang is intended to be a modular Solidity compiler, specifically targeting code analysis and developer tooling. 
//! This means servicing tools with domain-specific APIs and, in general, facilitating working with and analyzing the 
//! Solidity source code. If you're in the editor writing Solidity or performing linting or additional validation, 
//! there's a chance that you are, or could be, running Slang!
//!
//! # Parsing Solidity Source Code
//!
//! ```rs
//! use slang_solidity::parser::Parser;
//! use slang_solidity::LanguageFacts;
//! 
//! let source = "contract Foo {}";
//! let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
//! let parse_output = parser.parse_file_contents(&source);
//! ```
//!
//! 

mod extensions;
mod generated;

pub use generated::*;

pub mod backend;
