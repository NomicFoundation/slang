use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;

use super::super::nodes::NamedArgumentsStruct;
use crate::ast::Expression;

impl NamedArgumentsStruct {
    /// Reorders the named arguments into the positional order of `declaration_ids`
    /// (the callee's parameter / struct-field node ids), yielding each argument's
    /// value expression.
    ///
    /// Returns `None` if any argument name does not resolve to a definition, or if
    /// a `declaration_id` has no matching named argument.
    pub fn ordered_by(&self, declaration_ids: &[NodeId]) -> Option<Vec<Expression>> {
        let mut by_definition: HashMap<NodeId, Expression> = HashMap::new();
        for argument in self.iter() {
            let definition = argument.name().resolve_to_definition()?;
            by_definition.insert(definition.node_id(), argument.value());
        }
        declaration_ids
            .iter()
            .map(|declaration_id| by_definition.remove(declaration_id))
            .collect()
    }
}
