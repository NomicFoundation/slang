use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;

use super::super::nodes::NamedArgumentsStruct;
use crate::ast::Expression;

impl NamedArgumentsStruct {
    /// Reorders the named arguments into the positional order of `declaration_ids`
    /// (the callee's parameter / struct-field node ids), yielding each argument's
    /// value expression. The binder has already bound every declared name to
    /// exactly one argument.
    pub fn ordered_by(&self, declaration_ids: &[NodeId]) -> Vec<Expression> {
        let mut by_definition: HashMap<NodeId, Expression> = HashMap::new();
        for argument in self.iter() {
            let definition = argument
                .name()
                .resolve_to_definition()
                .expect("the binder resolves every named argument to its target definition");
            by_definition.insert(definition.node_id(), argument.value());
        }
        declaration_ids
            .iter()
            .map(|declaration_id| {
                by_definition
                    .remove(declaration_id)
                    .expect("the binder binds a named argument for every declared name")
            })
            .collect()
    }
}
