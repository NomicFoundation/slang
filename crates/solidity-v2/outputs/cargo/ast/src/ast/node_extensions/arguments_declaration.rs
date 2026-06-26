use slang_solidity_v2_common::nodes::NodeId;

use crate::ast::{ArgumentsDeclaration, Expression};

impl ArgumentsDeclaration {
    /// Orders a call / emit / revert argument list into the callee's
    /// parameter-declaration order, keyed by `parameter_ids`.
    ///
    /// Returns `None` for named arguments that cannot be matched to the given
    /// declarations (see [`NamedArgumentsStruct::ordered_by`]); positional
    /// arguments are always returned in source order.
    pub fn ordered_by(&self, parameter_ids: &[NodeId]) -> Option<Vec<Expression>> {
        match self {
            ArgumentsDeclaration::PositionalArguments(positional) => {
                Some(positional.iter().collect())
            }
            ArgumentsDeclaration::NamedArguments(named) => named.ordered_by(parameter_ids),
        }
    }
}
