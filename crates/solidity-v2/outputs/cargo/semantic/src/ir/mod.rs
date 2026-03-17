#[path = "nodes.generated.rs"]
mod nodes;
pub use nodes::*;

mod builder;

#[path = "default_builder.generated.rs"]
mod default_builder;

#[path = "visitor.generated.rs"]
pub mod visitor;

#[cfg(test)]
pub mod tests;

pub fn build(source_unit: &slang_solidity_v2_cst::structured_cst::nodes::SourceUnit) -> SourceUnit {
    use default_builder::Builder;

    let mut builder = builder::CstToIrBuilder {};
    builder.build_source_unit(source_unit)
}
