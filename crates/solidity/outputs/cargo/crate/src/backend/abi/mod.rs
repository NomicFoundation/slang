use std::ops::Div;

use sha3::{Digest, Keccak256};

use super::binder::Definition;
use super::ir::ast::{
    ContractDefinitionStruct, FunctionDefinitionStruct, FunctionKind, FunctionMutability,
    FunctionVisibility, ParametersStruct, StateVariableDefinitionStruct, StateVariableMutability,
    StateVariableVisibility,
};
use super::types::{Type, TypeId};
use super::SemanticAnalysis;
use crate::cst::NodeId;

pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub functions: Vec<FunctionAbi>,
    pub storage_layout: Vec<StorageItem>,
}

pub struct FunctionAbi {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub kind: FunctionKind,
    pub inputs: Vec<FunctionParameter>,
    pub outputs: Vec<FunctionParameter>,
    pub state_mutability: FunctionMutability,
}

pub struct FunctionParameter {
    pub node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    pub name: Option<String>,
    pub r#type: String,
}

pub struct StorageItem {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}

impl ContractDefinitionStruct {
    // TODO: ideally the user wouldn't need to provide the file_id and we should
    // be able to obtain it here, but for that we need bi-directional tree
    // navigation
    pub fn abi_with_file_id(&self, file_id: &str) -> Option<ContractAbi> {
        let name = self.name().unparse();
        let functions = self.abi_functions()?;
        let storage_layout = self.compute_storage_layout()?;
        Some(ContractAbi {
            node_id: self.ir_node.node_id,
            name,
            file_id: file_id.to_string(),
            functions,
            storage_layout,
        })
    }

    fn abi_functions(&self) -> Option<Vec<FunctionAbi>> {
        let mut functions = Vec::new();
        if let Some(constructor) = self.constructor() {
            functions.push(constructor.abi()?);
        }
        for function in &self.linearised_functions() {
            if function.is_public() {
                functions.push(function.abi()?);
            }
        }
        for state_variable in &self.linearised_state_variables() {
            if state_variable.is_public() {
                functions.push(state_variable.abi()?);
            }
        }

        functions.sort_by(|a, b| a.name.cmp(&b.name));
        Some(functions)
    }

    fn compute_storage_layout(&self) -> Option<Vec<StorageItem>> {
        let mut storage_layout = Vec::new();
        // TODO: if the contract has a specific storage layout specifier, we
        // need to compute its value and use it as the base
        let mut ptr: usize = 0;
        for state_variable in &self.linearised_state_variables() {
            let node_id = state_variable.ir_node.node_id;
            // skip constants and immutable variables, since they are not placed in storage
            // TODO: also, transient storage is laid out separately and we need
            // to support that as well
            if !matches!(
                state_variable.mutability(),
                StateVariableMutability::Mutable
            ) {
                continue;
            }

            let variable_type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;

            // check if we can pack the variable in the previous slot
            let remaining_bytes = SemanticAnalysis::SLOT_SIZE - (ptr % SemanticAnalysis::SLOT_SIZE);
            if variable_size > remaining_bytes {
                ptr += remaining_bytes;
            }

            let label = state_variable.name().unparse();
            let slot = ptr.div(SemanticAnalysis::SLOT_SIZE);
            let offset = ptr % SemanticAnalysis::SLOT_SIZE;
            let r#type = self.semantic.type_canonical_name(variable_type_id);
            storage_layout.push(StorageItem {
                node_id,
                label,
                slot,
                offset,
                r#type,
            });

            // ready pointer for the next variable
            ptr += variable_size;
        }
        Some(storage_layout)
    }
}

impl FunctionDefinitionStruct {
    pub fn is_public(&self) -> bool {
        matches!(
            self.visibility(),
            FunctionVisibility::Public | FunctionVisibility::External
        )
    }

    pub fn abi(&self) -> Option<FunctionAbi> {
        if !self.is_public() {
            return None;
        }
        let inputs = self.parameters().abi()?;
        let outputs = if let Some(returns) = self.returns() {
            returns.abi()?
        } else {
            Vec::new()
        };

        Some(FunctionAbi {
            node_id: self.ir_node.node_id,
            name: self.name().as_ref().map(|name| name.unparse()),
            kind: self.kind(),
            inputs,
            outputs,
            state_mutability: self.mutability(),
        })
    }

    pub fn selector(&self) -> Option<u32> {
        if !self.is_public() {
            return None;
        }
        let name = self.name()?;
        let signature = format!(
            "{name}({parameters})",
            name = name.unparse(),
            parameters = self.parameters().canonical_signature()?,
        );

        Some(selector_from_signature(&signature))
    }
}

impl ParametersStruct {
    pub(crate) fn abi(&self) -> Option<Vec<FunctionParameter>> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.node_id;
            let name = parameter.name.as_ref().map(|name| name.unparse());
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let r#type = self.semantic.type_canonical_name(type_id);
            result.push(FunctionParameter {
                node_id: Some(node_id),
                name,
                r#type,
            });
        }
        Some(result)
    }

    pub(crate) fn canonical_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.node_id;
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            result.push(self.semantic.type_canonical_name(type_id));
        }
        Some(result.join(","))
    }
}

impl StateVariableDefinitionStruct {
    pub fn is_public(&self) -> bool {
        matches!(self.visibility(), StateVariableVisibility::Public)
    }

    pub fn abi(&self) -> Option<FunctionAbi> {
        if !self.is_public() {
            return None;
        }
        let Some(Definition::StateVariable(definition)) = self
            .semantic
            .binder
            .find_definition_by_id(self.ir_node.node_id)
        else {
            return None;
        };
        let (inputs, outputs) = self
            .semantic
            .extract_function_type_parameters_abi(definition.getter_type_id?)?;

        Some(FunctionAbi {
            node_id: self.ir_node.node_id,
            name: Some(self.name().unparse()),
            kind: FunctionKind::Regular,
            inputs,
            outputs,
            state_mutability: FunctionMutability::View,
        })
    }

    pub fn selector(&self) -> Option<u32> {
        if !self.is_public() {
            return None;
        }
        let Some(Definition::StateVariable(definition)) = self
            .semantic
            .binder
            .find_definition_by_id(self.ir_node.node_id)
        else {
            return None;
        };
        let (inputs, _) = self
            .semantic
            .extract_function_type_parameters_abi(definition.getter_type_id?)?;

        let signature = format!(
            "{name}({parameters})",
            name = self.name().unparse(),
            parameters = inputs
                .into_iter()
                .map(|parameter| parameter.r#type)
                .collect::<Vec<_>>()
                .join(","),
        );

        Some(selector_from_signature(&signature))
    }
}

impl SemanticAnalysis {
    fn extract_function_type_parameters_abi(
        &self,
        type_id: TypeId,
    ) -> Option<(Vec<FunctionParameter>, Vec<FunctionParameter>)> {
        let Type::Function(function_type) = self.types.get_type_by_id(type_id) else {
            return None;
        };
        let inputs = function_type
            .parameter_types
            .iter()
            .map(|parameter_type_id| FunctionParameter {
                node_id: None,
                name: None,
                r#type: self.type_canonical_name(*parameter_type_id),
            })
            .collect();
        let outputs = vec![FunctionParameter {
            node_id: None,
            name: None,
            r#type: self.type_canonical_name(function_type.return_type),
        }];
        Some((inputs, outputs))
    }
}

fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let result = hasher.finalize();

    let selector_bytes: [u8; 4] = result[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}
