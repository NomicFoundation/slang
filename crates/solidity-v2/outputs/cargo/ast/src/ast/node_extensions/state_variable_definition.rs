use slang_solidity_v2_semantic::binder;

use super::super::{Definition, StateVariableDefinitionStruct, StructMember, Type};

impl StateVariableDefinitionStruct {
    /// Returns the type of the getter generated for this state variable, or
    /// `None` if it is not public.
    pub fn getter_type(&self) -> Option<Type> {
        let binder::Definition::StateVariable(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a state variable");
        };
        Some(Type::create(definition.getter_type_id?, &self.semantic))
    }

    /// Returns the struct members the getter's return type is built from, in
    /// declaration order, or an empty list if it is not built from a struct.
    pub fn getter_struct_members(&self) -> Vec<StructMember> {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("definition is not a state variable");
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
