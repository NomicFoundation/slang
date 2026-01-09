#![allow(missing_docs)]

pub mod abi;
pub mod binder;
pub mod built_ins;
pub mod ir;
pub mod passes;
pub mod semantic;
pub mod types;

pub use semantic::{SemanticAnalysis, SemanticFile};
