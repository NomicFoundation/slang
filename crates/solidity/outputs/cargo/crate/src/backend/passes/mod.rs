use crate::compilation::CompilationUnit;

pub mod p0_build_ast;
pub mod p1_flatten_contracts;
pub mod p2_collect_definitions;
pub mod p3_linearise_contracts;
pub mod p4_type_definitions;
pub mod p5_resolve_references;
pub mod p6_index_tree;

pub use crate::backend::ir::ir2_flat_contracts as ast;

// TODO: likely we want to be more specific here
pub type CompilationOutput = p6_index_tree::Output;

pub fn compile(unit: CompilationUnit) -> CompilationOutput {
    let data = p0_build_ast::run(unit);
    let data = p1_flatten_contracts::run(data);
    let data = p2_collect_definitions::run(data);
    let data = p3_linearise_contracts::run(data);
    let data = p4_type_definitions::run(data);
    let data = p5_resolve_references::run(data);
    p6_index_tree::run(data)
}
