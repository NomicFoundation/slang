use slang_solidity_v2_semantic::ir;

use super::super::{FunctionDefinition, FunctionDefinitionStruct};
use crate::abi::{
    selector_from_signature, type_as_abi_parameter, AbiEntry, AbiMutability, AbiParameter,
};
use crate::ast::{FunctionVisibility, ParametersStruct};

impl FunctionDefinitionStruct {
    pub(crate) fn overrides(&self, other: &FunctionDefinition) -> bool {
        let name_matches = match (&self.ir_node.name, &other.ir_node.name) {
            (None, None) => {
                // for unnamed functions, we check the kind because `receive`
                // and `fallback` may have the same parameters but they are
                // different functions
                self.ir_node.kind == other.ir_node.kind
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

// ABI-related extensions
impl FunctionDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.visibility(),
            FunctionVisibility::Public | FunctionVisibility::External
        )
    }

    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let inputs = self.parameters().compute_abi_parameters()?;
        let outputs = if let Some(returns) = self.returns() {
            returns.compute_abi_parameters()?
        } else {
            Vec::new()
        };

        let node_id = self.ir_node.id();
        let name = self
            .ir_node
            .name
            .as_ref()
            .map(|name| name.unparse().to_string());
        let state_mutability: AbiMutability = self.ir_node.mutability.clone();

        match self.ir_node.kind {
            ir::FunctionKind::Regular => Some(AbiEntry::Function {
                node_id,
                name: name?,
                inputs,
                outputs,
                state_mutability,
            }),
            ir::FunctionKind::Constructor => Some(AbiEntry::Constructor {
                node_id,
                inputs,
                state_mutability,
            }),
            ir::FunctionKind::Fallback => Some(AbiEntry::Fallback {
                node_id,
                state_mutability,
            }),
            ir::FunctionKind::Receive => Some(AbiEntry::Receive {
                node_id,
                state_mutability,
            }),
            ir::FunctionKind::Modifier => None,
        }
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let name = self.ir_node.name.as_ref()?.unparse();
        let signature = format!(
            "{name}({parameters})",
            parameters = self.parameters().compute_canonical_signature()?,
        );

        Some(selector_from_signature(&signature))
    }
}

impl ParametersStruct {
    pub(crate) fn compute_abi_parameters(&self) -> Option<Vec<AbiParameter>> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.id();
            let name = parameter
                .name
                .as_ref()
                .map(|name| name.unparse().to_string());
            let indexed = parameter.indexed;
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let (r#type, components) = type_as_abi_parameter(&self.semantic, type_id)?;
            result.push(AbiParameter {
                node_id: Some(node_id),
                name,
                r#type,
                components,
                indexed,
            });
        }
        Some(result)
    }

    pub(crate) fn compute_canonical_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.id();
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            result.push(self.semantic.type_canonical_name(type_id)?);
        }
        Some(result.join(","))
    }
}
