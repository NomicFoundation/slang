use super::super::{ContractMember, ErrorDefinition, EventDefinition, InterfaceMembersStruct};

impl InterfaceMembersStruct {
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
