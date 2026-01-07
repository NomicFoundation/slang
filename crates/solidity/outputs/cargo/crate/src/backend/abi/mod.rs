use super::binder::Definition;
use super::ir::ast::{
    ContractDefinitionStruct, FunctionDefinitionStruct, FunctionKind, FunctionMutability,
    FunctionVisibility, ParametersStruct, StateVariableDefinitionStruct, StateVariableVisibility,
};
use super::types::{Type, TypeId};
use super::SemanticAnalysis;
use crate::cst::NodeId;

pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub functions: Vec<FunctionAbi>,
    pub storage_layout: Vec<Slot>,
}

pub struct FunctionAbi {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub kind: FunctionKind,
    pub inputs: Vec<FunctionInputOutput>,
    pub outputs: Vec<FunctionInputOutput>,
    pub state_mutability: FunctionMutability,
}

pub struct FunctionInputOutput {
    pub node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    pub name: Option<String>,
    pub r#type: String,
}

pub struct Slot {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}

impl ContractDefinitionStruct {
    pub(crate) fn abi_with_file_id(&self, file_id: &str) -> ContractAbi {
        let name = self.name().unparse();
        let functions = self.abi_functions();
        ContractAbi {
            node_id: self.ir_node.node_id,
            name,
            file_id: file_id.to_string(),
            functions,
            storage_layout: Vec::new(), // TODO
        }
    }

    fn abi_functions(&self) -> Vec<FunctionAbi> {
        let mut functions = Vec::new();
        if let Some(constructor) = self.constructor() {
            if let Some(abi_function) = constructor.abi() {
                functions.push(abi_function);
            }
        }
        for function in &self.linearised_functions() {
            if let Some(abi_function) = function.abi() {
                functions.push(abi_function);
            }
        }
        for state_variable in &self.linearised_state_variables() {
            if let Some(abi_function) = state_variable.abi() {
                functions.push(abi_function);
            }
        }

        functions.sort_by(|a, b| a.name.cmp(&b.name));
        functions
    }
}

impl FunctionDefinitionStruct {
    pub(crate) fn abi(&self) -> Option<FunctionAbi> {
        if !matches!(
            self.visibility(),
            FunctionVisibility::Public | FunctionVisibility::External
        ) {
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
}

impl ParametersStruct {
    pub(crate) fn abi(&self) -> Option<Vec<FunctionInputOutput>> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.node_id;
            let name = parameter.name.as_ref().map(|name| name.unparse());
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let r#type = self.semantic.type_canonical_name(type_id);
            result.push(FunctionInputOutput {
                node_id: Some(node_id),
                name,
                r#type,
            });
        }
        Some(result)
    }
}

impl StateVariableDefinitionStruct {
    pub(crate) fn abi(&self) -> Option<FunctionAbi> {
        if !matches!(self.visibility(), StateVariableVisibility::Public) {
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
}

impl SemanticAnalysis {
    fn extract_function_type_parameters_abi(
        &self,
        type_id: TypeId,
    ) -> Option<(Vec<FunctionInputOutput>, Vec<FunctionInputOutput>)> {
        let Type::Function(function_type) = self.types.get_type_by_id(type_id) else {
            return None;
        };
        let inputs = function_type
            .parameter_types
            .iter()
            .map(|parameter_type_id| FunctionInputOutput {
                node_id: None,
                name: None,
                r#type: self.type_canonical_name(*parameter_type_id),
            })
            .collect();
        let outputs = vec![FunctionInputOutput {
            node_id: None,
            name: None,
            r#type: self.type_canonical_name(function_type.return_type),
        }];
        Some((inputs, outputs))
    }
}
