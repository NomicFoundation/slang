use super::super::{
    ContractMember, ContractMembersStruct, ErrorDefinition, EventDefinition, FunctionDefinition,
    InterfaceMembersStruct, LibraryMembersStruct, StateVariableDefinition,
};

/// A contract's, an interface's and a library's member lists are all lists of
/// `ContractMember`, so one iterator body serves every member kind each admits.
macro_rules! define_member_iterators {
    ($list:ident { $($method:ident($member:ident),)+ }) => {
        impl $list {
            $(
                pub(crate) fn $method(&self) -> impl Iterator<Item = $member> + use<'_> {
                    self.iter().filter_map(|member| {
                        if let ContractMember::$member(definition) = member {
                            Some(definition)
                        } else {
                            None
                        }
                    })
                }
            )+
        }
    };
}

define_member_iterators!(ContractMembersStruct {
    iter_function_definitions(FunctionDefinition),
    iter_state_variable_definitions(StateVariableDefinition),
    iter_error_definitions(ErrorDefinition),
    iter_event_definitions(EventDefinition),
});

define_member_iterators!(InterfaceMembersStruct {
    iter_function_definitions(FunctionDefinition),
});

define_member_iterators!(LibraryMembersStruct {
    iter_function_definitions(FunctionDefinition),
    iter_state_variable_definitions(StateVariableDefinition),
});
