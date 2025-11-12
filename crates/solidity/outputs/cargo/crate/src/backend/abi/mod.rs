use super::ir::ast::{
    ContractDefinitionStruct, FunctionKind, FunctionMutability, FunctionVisibility,
    StateVariableVisibility,
};
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
    pub node_id: NodeId,
    pub name: Option<String>,
    pub r#type: String,
    pub internal_type: String,
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
            let function = FunctionAbi {
                node_id: constructor.ir_node.node_id,
                name: None,
                kind: constructor.kind(),
                inputs: Vec::new(),  // TODO
                outputs: Vec::new(), // TODO
                state_mutability: constructor.mutability(),
            };
            functions.push(function);
        }
        for function in &self.linearised_functions() {
            if !matches!(
                function.visibility(),
                FunctionVisibility::Public | FunctionVisibility::External
            ) {
                continue;
            }

            let kind = function.kind();
            let state_mutability = function.mutability();
            let abi_function = FunctionAbi {
                node_id: function.ir_node.node_id,
                name: function.name().as_ref().map(|name| name.unparse()),
                kind,
                inputs: Vec::new(),  // TODO
                outputs: Vec::new(), // TODO
                state_mutability,
            };
            functions.push(abi_function);
        }
        for state_variable in &self.linearised_state_variables() {
            if !matches!(state_variable.visibility(), StateVariableVisibility::Public) {
                continue;
            }
            let abi_function = FunctionAbi {
                node_id: state_variable.ir_node.node_id,
                name: Some(state_variable.name().unparse()),
                kind: FunctionKind::Regular,
                inputs: Vec::new(),  // TODO
                outputs: Vec::new(), // TODO
                state_mutability: FunctionMutability::View,
            };
            functions.push(abi_function);
        }

        functions.sort_by(|a, b| a.name.cmp(&b.name));
        functions
    }
}
