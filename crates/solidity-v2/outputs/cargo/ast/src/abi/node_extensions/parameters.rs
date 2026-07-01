use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::types::type_as_abi_type;
use crate::abi::AbiParameter;
use crate::ast::ParametersStruct;

impl ParametersStruct {
    pub(crate) fn compute_abi_parameters(&self) -> Option<Vec<AbiParameter>> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.id();
            let name = parameter
                .name
                .as_ref()
                .map(|name| name.unparse().to_string());
            let indexed = parameter.is_indexed;
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let abi_type = type_as_abi_type(&self.semantic, type_id)?;
            result.push(AbiParameter {
                node_id: Some(node_id),
                name,
                abi_type,
                indexed,
            });
        }
        Some(result)
    }

    fn parameter_types_iter(&self) -> impl Iterator<Item = Option<TypeId>> + '_ {
        self.ir_nodes.iter().map(|parameter| {
            self.semantic
                .binder()
                .node_typing(parameter.id())
                .as_type_id()
        })
    }

    pub(crate) fn compute_canonical_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for type_id in self.parameter_types_iter() {
            let abi_type = type_as_abi_type(&self.semantic, type_id?)?;
            result.push(abi_type.to_string());
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
