use slang_solidity_v2_common::nodes::NodeId;

use crate::ast::ArgumentsDeclaration;
use crate::ast::Expression;

impl ArgumentsDeclaration {
    /// Orders a call / emit / revert argument list into the callee's
    /// parameter-declaration order, keyed by `parameter_ids` (node-id identity,
    /// never name text). Positional arguments are already in order; named
    /// arguments are reordered into parameter order.
    pub fn ordered_by(&self, parameter_ids: &[NodeId]) -> Vec<Expression> {
        match self {
            ArgumentsDeclaration::PositionalArguments(positional) => positional.iter().collect(),
            ArgumentsDeclaration::NamedArguments(named) => named.ordered_by(parameter_ids),
        }
    }
}
