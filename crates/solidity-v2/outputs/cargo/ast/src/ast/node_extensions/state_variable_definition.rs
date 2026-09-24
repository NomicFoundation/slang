use slang_solidity_v2_common::nodes::NodeId;
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

    pub(crate) fn getter_input_definition_ids(&self) -> &[Option<NodeId>] {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("state variable node without a state variable definition");
        };
        &definition.getter_input_definition_ids
    }

    pub(crate) fn getter_output_definition_ids(&self) -> &[Option<NodeId>] {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("state variable node without a state variable definition");
        };
        &definition.getter_output_definition_ids
    }

    /// Returns the type of the getter generated for this state variable, or
    /// `None` if it is not public.
    pub fn getter_type(&self) -> Option<Type> {
        Some(Type::create(self.getter_type_id()?, &self.semantic))
    }

    /// Returns the struct members the getter's return type is built from, in
    /// declaration order, or an empty list if it is not built from a struct.
    pub fn getter_struct_members(&self) -> Vec<StructMember> {
        self.getter_output_definition_ids()
            .iter()
            .filter_map(
                |output_id| match Definition::try_create((*output_id)?, &self.semantic) {
                    Some(Definition::StructMember(member)) => Some(member),
                    Some(_) => None,
                    None => unreachable!("getter output without a definition"),
                },
            )
            .collect()
    }
}
