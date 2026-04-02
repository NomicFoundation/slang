use super::super::FunctionCallExpressionStruct;
use crate::backend::binder::Typing;
use crate::backend::ir::ir2_flat_contracts as input_ir;

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            input_ir::Expression::ElementaryType(_) | input_ir::Expression::PayableKeyword => true,
            input_ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder.try_node_typing(terminal.id()),
                Some(Typing::MetaType(_) | Typing::UserMetaType(_))
            ),
            input_ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder.try_node_typing(mae.node_id),
                Some(Typing::MetaType(_) | Typing::UserMetaType(_))
            ),
            _ => false,
        }
    }
}
