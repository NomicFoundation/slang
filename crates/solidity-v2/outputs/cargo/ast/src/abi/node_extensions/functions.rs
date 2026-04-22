use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::{
    selector_from_signature, type_as_abi_parameter, AbiConstructor, AbiEntry, AbiFallback,
    AbiFunction, AbiMutability, AbiParameter, AbiReceive,
};
use crate::ast::{FunctionDefinitionStruct, FunctionVisibility, ParametersStruct};

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
        let state_mutability: AbiMutability = self.ir_node.mutability;

        match self.ir_node.kind {
            ir::FunctionKind::Regular => Some(AbiEntry::Function(AbiFunction {
                node_id,
                name: name?,
                inputs,
                outputs,
                state_mutability,
            })),
            ir::FunctionKind::Constructor => Some(AbiEntry::Constructor(AbiConstructor {
                node_id,
                inputs,
                state_mutability,
            })),
            ir::FunctionKind::Fallback => Some(AbiEntry::Fallback(AbiFallback {
                node_id,
                state_mutability,
            })),
            ir::FunctionKind::Receive => Some(AbiEntry::Receive(AbiReceive {
                node_id,
                state_mutability,
            })),
            ir::FunctionKind::Modifier => None,
        }
    }

    /// Returns the external signature for this function, suitable for ABI encoding.
    ///
    /// This is only guaranteed for external functions in valid Solidity, as
    /// internal functions may contain parameter types that cannot be
    /// ABI-encoded.
    pub fn compute_canonical_signature(&self) -> Option<String> {
        let name = self.ir_node.name.as_ref()?.unparse();
        let parameters = self.parameters().compute_canonical_signature()?;
        Some(format!("{name}({parameters})"))
    }

    /// Returns the signature for this function using internal type names for
    /// parameters. Unlike [`Self::compute_canonical_signature`], this form is
    /// well-defined for any function, including internal ones with parameter
    /// types that cannot be ABI-encoded.
    pub fn compute_internal_signature(&self) -> Option<String> {
        let name = self.ir_node.name.as_ref()?.unparse();
        let parameters = self.parameters().compute_internal_signature()?;
        Some(format!("{name}({parameters})"))
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        self.compute_canonical_signature()
            .map(|sig| selector_from_signature(&sig))
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
            let (type_name, components) = type_as_abi_parameter(&self.semantic, type_id)?;
            result.push(AbiParameter {
                node_id: Some(node_id),
                name,
                type_name,
                components,
                indexed,
            });
        }
        Some(result)
    }

    fn parameter_types_iter(&self) -> impl Iterator<Item = Option<TypeId>> + '_ {
        self.ir_nodes.iter().map(|parameter| {
            self.semantic.binder().node_typing(parameter.id()).as_type_id()
        })
    }

    pub(crate) fn compute_canonical_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for type_id in self.parameter_types_iter() {
            result.push(self.semantic.type_canonical_name(type_id?)?);
        }
        Some(result.join(","))
    }

    pub(crate) fn compute_internal_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for type_id in self.parameter_types_iter() {
            result.push(self.semantic.type_internal_name(type_id?));
        }
        Some(result.join(","))
    }
}
