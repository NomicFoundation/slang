use super::super::Type;
use crate::ast::{
    AdditiveExpressionStruct, AndExpressionStruct, BitwiseAndExpressionStruct,
    BitwiseOrExpressionStruct, BitwiseXorExpressionStruct, EqualityExpressionStruct,
    ExponentiationExpressionStruct, InequalityExpressionStruct, MultiplicativeExpressionStruct,
    OrExpressionStruct, ShiftExpressionStruct,
};

/// A binary operator expression, whose two operands are reconciled to a common
/// type before the operator applies.
pub trait BinaryOperatorExpression {
    /// The common type both operands reconcile to before the operator applies;
    /// `None` when they have no common type.
    fn common_operand_type(&self) -> Option<Type>;
}

/// The comparison operators are the only binary operators whose common operand
/// type differs from their (boolean) result, so the typing pass records it and
/// the AST reads it back from the binder.
macro_rules! impl_recorded_common_operand_type {
    ($($node:ident),* $(,)?) => {
        $(
            impl BinaryOperatorExpression for $node {
                fn common_operand_type(&self) -> Option<Type> {
                    let type_id = self
                        .semantic
                        .binder()
                        .common_operand_typing(self.ir_node.id())
                        .as_type_id()?;
                    Some(Type::create(type_id, &self.semantic))
                }
            }
        )*
    };
}

/// Every other binary operator reconciles its operands to its own result type,
/// so the common operand type is just the node's type — no separate record.
macro_rules! impl_result_common_operand_type {
    ($($node:ident),* $(,)?) => {
        $(
            impl BinaryOperatorExpression for $node {
                fn common_operand_type(&self) -> Option<Type> {
                    self.get_type()
                }
            }
        )*
    };
}

impl_recorded_common_operand_type!(EqualityExpressionStruct, InequalityExpressionStruct);

impl_result_common_operand_type!(
    AdditiveExpressionStruct,
    AndExpressionStruct,
    BitwiseAndExpressionStruct,
    BitwiseOrExpressionStruct,
    BitwiseXorExpressionStruct,
    ExponentiationExpressionStruct,
    MultiplicativeExpressionStruct,
    OrExpressionStruct,
    ShiftExpressionStruct,
);
