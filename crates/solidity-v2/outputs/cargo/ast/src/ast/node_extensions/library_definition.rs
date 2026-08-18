use super::super::{
    FunctionDefinition, FunctionKind, LibraryDefinitionStruct, StateVariableDefinition,
};

impl LibraryDefinitionStruct {
    pub fn state_variables(&self) -> Vec<StateVariableDefinition> {
        self.members().iter_state_variable_definitions().collect()
    }

    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| matches!(function.kind(), FunctionKind::Regular))
            .collect()
    }
}
