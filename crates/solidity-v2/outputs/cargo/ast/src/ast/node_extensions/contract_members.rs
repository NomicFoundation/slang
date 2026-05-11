use super::super::{
    ContractMember, ContractMembersStruct, ErrorDefinition, EventDefinition, FunctionDefinition,
    StateVariableDefinition,
};

impl ContractMembersStruct {
    pub(crate) fn iter_function_definitions(
        &self,
    ) -> impl Iterator<Item = FunctionDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::FunctionDefinition(function) = member {
                Some(function)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_state_variable_definitions(
        &self,
    ) -> impl Iterator<Item = StateVariableDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::StateVariableDefinition(state_variable) = member {
                Some(state_variable)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_error_definitions(&self) -> impl Iterator<Item = ErrorDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::ErrorDefinition(error_definition) = member {
                Some(error_definition)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_event_definitions(&self) -> impl Iterator<Item = EventDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::EventDefinition(event_definition) = member {
                Some(event_definition)
            } else {
                None
            }
        })
    }
}
