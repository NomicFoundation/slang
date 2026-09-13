use super::super::nodes::{
    create_error_definition, create_event_definition, create_function_definition,
};
use super::super::{
    Definition, ErrorDefinition, EventDefinition, FunctionDefinition, FunctionKind,
    InterfaceDefinitionStruct,
};
use super::ContractBase;

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

    /// Returns the list of interfaces in the hierarchy (including self) in the
    /// order given by the C3 linearisation, with self interface always first.
    /// A contract base, which solc rejects, is kept as `ContractBase::Contract`.
    pub fn linearised_bases(&self) -> Vec<ContractBase> {
        let Some(base_node_ids) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.id())
        else {
            // TODO(validation) SDR[4]: once we have validation implemented, this
            // branch should not be reachable, or we should generate an error
            // while building the `SemanticAnalysis`.
            return Vec::new();
        };
        base_node_ids
            .iter()
            .map(|node_id| {
                let base_definition =
                    Definition::try_create(*node_id, &self.semantic).expect("node is a definition");
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
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
