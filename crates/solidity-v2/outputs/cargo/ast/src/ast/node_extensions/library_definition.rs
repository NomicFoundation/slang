use super::super::nodes::{
    create_error_definition, create_event_definition, create_function_definition,
};
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

    /// The functions this library's code runs: its external and public
    /// functions, the functions they call, and its pointer targets.
    pub fn runtime_functions(&self) -> Vec<FunctionDefinition> {
        self.semantic
            .runtime_functions(self.ir_node.id())
            .iter()
            .map(|ir_node| create_function_definition(ir_node, &self.semantic))
            .collect()
    }

    /// The runtime functions a call through an internal function value in
    /// this library's code can reach, ie. the functions that code takes as
    /// values with the signature of one of its pointer calls.
    pub fn runtime_pointer_targets(&self) -> Vec<FunctionDefinition> {
        self.semantic
            .runtime_pointer_targets(self.ir_node.id())
            .iter()
            .map(|ir_node| create_function_definition(ir_node, &self.semantic))
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
