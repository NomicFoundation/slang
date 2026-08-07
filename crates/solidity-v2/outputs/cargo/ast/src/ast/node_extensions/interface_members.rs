use super::super::{ContractMember, FunctionDefinition, InterfaceMembersStruct};

impl InterfaceMembersStruct {
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
}
