use super::super::nodes::{
    create_error_definition, create_event_definition, create_function_definition,
    create_state_variable_definition,
};
use super::super::{
    ContractDefinitionStruct, Definition, ErrorDefinition, EventDefinition, FunctionDefinition,
    FunctionKind, FunctionMutability, StateVariableDefinition,
};
use super::ContractBase;

impl ContractDefinitionStruct {
    pub fn direct_bases(&self) -> Vec<ContractBase> {
        self.inheritance_types()
            .iter()
            .filter_map(|inheritance_type| {
                let base = inheritance_type.type_name().resolve_to_definition()?;
                ContractBase::from_definition(&base)
            })
            .collect()
    }

    /// Returns the list of contracts/interfaces in the hierarchy (including
    /// self) in the order given by the C3 linearisation, with self contract
    /// always first
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

    pub fn state_variables(&self) -> Vec<StateVariableDefinition> {
        self.members().iter_state_variable_definitions().collect()
    }

    /// Returns the list of state variable definitions in the order laid out in storage
    pub fn linearised_state_variables(&self) -> Vec<StateVariableDefinition> {
        self.semantic
            .linearised_state_variables(self.ir_node.id())
            .iter()
            .map(|ir_node| create_state_variable_definition(ir_node, &self.semantic))
            .collect()
    }

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

    /// Whether the contract accepts a plain ETH transfer: it, or any base in
    /// its linearisation, declares a `receive()` function or a `payable`
    /// `fallback()` (solc's `ContractType::isPayable`).
    pub fn is_payable(&self) -> bool {
        self.linearised_functions().iter().any(|function| {
            matches!(function.kind(), FunctionKind::Receive)
                || (matches!(function.kind(), FunctionKind::Fallback)
                    && matches!(
                        function.attributes().mutability(),
                        FunctionMutability::Payable
                    ))
        })
    }

    pub fn modifiers(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| matches!(function.kind(), FunctionKind::Modifier))
            .collect()
    }

    pub fn constructor(&self) -> Option<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .find(|function| matches!(function.kind(), FunctionKind::Constructor))
    }

    /// Returns the list of functions defined in all the hierarchy of the
    /// contract, in alphabetical order
    pub fn linearised_functions(&self) -> Vec<FunctionDefinition> {
        self.semantic
            .linearised_functions(self.ir_node.id())
            .iter()
            .map(|ir_node| create_function_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn errors(&self) -> Vec<ErrorDefinition> {
        self.members().iter_error_definitions().collect()
    }

    pub fn linearised_errors(&self) -> Vec<ErrorDefinition> {
        self.semantic
            .linearised_errors(self.ir_node.id())
            .iter()
            .map(|ir_node| create_error_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn events(&self) -> Vec<EventDefinition> {
        self.members().iter_event_definitions().collect()
    }

    pub fn linearised_events(&self) -> Vec<EventDefinition> {
        self.semantic
            .linearised_events(self.ir_node.id())
            .iter()
            .map(|ir_node| create_event_definition(ir_node, &self.semantic))
            .collect()
    }
}
