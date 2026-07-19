use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::super::Type;
use crate::ast::{EqualityExpressionStruct, InequalityExpressionStruct};

/// The common type both operands of a comparison reconcile to before it runs,
/// as recorded by the typing pass; `None` when the pass recorded none.
fn common_operand_type_of(node_id: NodeId, semantic: &Arc<SemanticContext>) -> Option<Type> {
    let type_id = semantic
        .binder()
        .comparison_operand_typing(node_id)
        .as_type_id()?;
    Some(Type::create(type_id, semantic))
}

impl EqualityExpressionStruct {
    /// The common type both operands are reconciled to before the `==` / `!=`
    /// comparison, as recorded by the typing pass.
    pub fn common_operand_type(&self) -> Option<Type> {
        common_operand_type_of(self.ir_node.id(), &self.semantic)
    }
}

impl InequalityExpressionStruct {
    /// The common type both operands are reconciled to before the ordering
    /// comparison, as recorded by the typing pass.
    pub fn common_operand_type(&self) -> Option<Type> {
        common_operand_type_of(self.ir_node.id(), &self.semantic)
    }
}
