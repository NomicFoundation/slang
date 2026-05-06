use slang_solidity_v2_ir::ir;

use super::super::{FunctionDefinition, FunctionDefinitionStruct};

impl FunctionDefinitionStruct {
    pub(crate) fn overrides(&self, other: &FunctionDefinition) -> bool {
        let name_matches = match (&self.ir_node.name, &other.ir_node.name) {
            (None, None) => {
                // for unnamed functions, we check the kind because `receive`
                // and `fallback` may have the same parameters but they are
                // different functions
                matches!(
                    (&self.ir_node.kind, &other.ir_node.kind),
                    (ir::FunctionKind::Constructor, ir::FunctionKind::Constructor)
                        | (ir::FunctionKind::Fallback, ir::FunctionKind::Fallback)
                        | (ir::FunctionKind::Modifier, ir::FunctionKind::Modifier)
                        | (ir::FunctionKind::Receive, ir::FunctionKind::Receive)
                        | (ir::FunctionKind::Regular, ir::FunctionKind::Regular)
                )
            }
            (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
            _ => false,
        };
        if !name_matches {
            return false;
        }
        let type_id = self
            .semantic
            .binder()
            .node_typing(self.ir_node.id())
            .as_type_id();
        let other_type_id = self
            .semantic
            .binder()
            .node_typing(other.ir_node.id())
            .as_type_id();

        match (type_id, other_type_id) {
            (Some(type_id), Some(other_type_id)) => self
                .semantic
                .types()
                .type_id_is_function_and_overrides(type_id, other_type_id),
            _ => false,
        }

        // TODO(validation): check also that the function mutability is stricter than other's
    }
}
