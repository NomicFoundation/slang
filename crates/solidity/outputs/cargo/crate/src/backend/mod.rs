#![allow(missing_docs)]

use crate::compilation::CompilationUnit;

pub mod l1_structured_ast;
pub mod l2_flat_contracts;

pub mod binder;
pub mod built_ins;
pub mod passes;
pub mod types;

pub type BinderOutput = passes::p5_resolve_references::Output;

pub fn build_binder_output(compilation_unit: CompilationUnit) -> BinderOutput {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);
    let data = passes::p4_type_definitions::run(data);
    passes::p5_resolve_references::run(data)
}
