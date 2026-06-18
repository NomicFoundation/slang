use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::ScopeId;

/// An `assembly { ... }` statement recorded during `p1_collect_definitions`.
///
/// Collecting these up front lets `p6_resolve_yul` process only the tree
/// branches that actually contain Yul (instead of walking every file's full IR
/// tree), and gives the compiler backend a per-block record of which Solidity
/// definitions each assembly block references.
#[derive(Debug)]
pub(crate) struct AssemblyBlock {
    /// The `assembly { ... }` statement node.
    pub(crate) ir_node: ir::AssemblyStatement,
    /// Enclosing Solidity scope active at the statement. `p6_resolve_yul` uses
    /// it as the parent of the Yul block scope and as the head of the
    /// resolution chain.
    pub(crate) enclosing_scope_id: ScopeId,
    /// Distinct Solidity definitions referenced anywhere inside the block.
    /// Empty until populated by `p6_resolve_yul`.
    pub(crate) solidity_references: Vec<NodeId>,
}
