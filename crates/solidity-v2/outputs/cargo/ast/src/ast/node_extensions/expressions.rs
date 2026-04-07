use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;

use super::super::FunctionCallExpressionStruct;

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            ir::Expression::ElementaryType(_) | ir::Expression::PayableKeyword => true,
            ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder().node_typing(terminal.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder().node_typing(mae.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            _ => false,
        }
    }
}
