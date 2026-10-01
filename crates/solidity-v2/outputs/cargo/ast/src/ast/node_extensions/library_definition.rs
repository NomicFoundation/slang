use super::super::nodes::{create_error_definition, create_event_definition};
use super::super::{
    ErrorDefinition, EventDefinition, FunctionDefinition, FunctionKind, LibraryDefinitionStruct,
    StateVariableDefinition,
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

    /// The errors this library's external and public functions can revert
    /// with, directly or through the code they reach, wherever they are
    /// declared.
    pub fn used_errors(&self) -> Vec<ErrorDefinition> {
        self.semantic
            .used_errors(self.ir_node.id())
            .iter()
            .map(|ir_node| create_error_definition(ir_node, &self.semantic))
            .collect()
    }

    /// The events this library's external and public functions can emit,
    /// directly or through the code they reach, wherever they are declared.
    pub fn used_events(&self) -> Vec<EventDefinition> {
        self.semantic
            .used_events(self.ir_node.id())
            .iter()
            .map(|ir_node| create_event_definition(ir_node, &self.semantic))
            .collect()
    }
}
