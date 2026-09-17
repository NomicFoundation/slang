use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::types::TypeId;

use super::super::{Definition, StateVariableDefinitionStruct, StructMember, Type};

impl StateVariableDefinitionStruct {
    pub(crate) fn getter_type_id(&self) -> Option<TypeId> {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("state variable node without a state variable definition");
        };
        definition.getter_type_id
    }

    /// Returns the type of the getter generated for this state variable, or
    /// `None` if it is not public.
    pub fn getter_type(&self) -> Option<Type> {
        Some(Type::create(self.getter_type_id()?, &self.semantic))
    }

    /// Returns the struct members the getter's return type is built from, in
    /// declaration order, or an empty list if it is not built from a struct.
    pub fn getter_struct_members(&self) -> Vec<StructMember> {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("state variable node without a state variable definition");
        };
        definition
            .getter_member_ids
            .iter()
            .map(|member_id| {
                let Some(Definition::StructMember(member)) =
                    Definition::try_create(*member_id, &self.semantic)
                else {
                    unreachable!("getter member is not a struct member");
                };
                member
            })
            .collect()
    }
}
