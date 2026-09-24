use super::super::nodes::{
    create_error_definition, create_event_definition, create_function_definition,
};
use super::super::{
    ErrorDefinition, EventDefinition, FunctionDefinition, FunctionKind, InterfaceDefinitionStruct,
};

impl InterfaceDefinitionStruct {
    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| {
                matches!(
                    function.kind(),
                    FunctionKind::Regular | FunctionKind::Fallback | FunctionKind::Receive
                )
            })
            .collect()
    }

    /// Returns the list of functions declared in all the hierarchy of the
    /// interface, in alphabetical order, an overriding declaration standing in
    /// for the one it overrides.
    pub fn linearised_functions(&self) -> Vec<FunctionDefinition> {
        self.semantic
            .linearised_functions(self.ir_node.id())
            .iter()
            .map(|ir_node| create_function_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn linearised_errors(&self) -> Vec<ErrorDefinition> {
        self.semantic
            .linearised_errors(self.ir_node.id())
            .iter()
            .map(|ir_node| create_error_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn linearised_events(&self) -> Vec<EventDefinition> {
        self.semantic
            .linearised_events(self.ir_node.id())
            .iter()
            .map(|ir_node| create_event_definition(ir_node, &self.semantic))
            .collect()
    }
}
