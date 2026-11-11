use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::types::TypeId;

use super::super::{Definition, StateVariableDefinitionStruct, StructMember, Type};

impl StateVariableDefinitionStruct {
    fn getter(&self) -> Option<&binder::Getter> {
        let Some(binder::Definition::StateVariable(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("state variable node without a state variable definition");
        };
        definition.getter.as_ref()
    }

    pub(crate) fn getter_type_id(&self) -> Option<TypeId> {
        self.getter().map(|getter| getter.type_id)
    }

    /// The getter's inputs, one per mapping key or array index, or `None` if
    /// the variable has no getter.
    pub(crate) fn getter_parameters(&self) -> Option<&[binder::ParameterDefinition]> {
        self.semantic
            .binder()
            .get_parameters_for_definition(self.ir_node.id())
    }

    /// The getter's outputs, matching its return type, or `None` if the
    /// variable has no getter.
    pub(crate) fn getter_return_parameters(&self) -> Option<&[binder::ParameterDefinition]> {
        self.getter()
            .map(|getter| getter.return_parameters.as_slice())
    }

    /// Returns the type of the getter generated for this state variable, or
    /// `None` if it is not public.
    pub fn getter_type(&self) -> Option<Type> {
        Some(Type::create(self.getter_type_id()?, &self.semantic))
    }

    /// Returns the struct members the getter's return type is built from, in
    /// declaration order, or an empty list if it is not built from a struct.
    pub fn getter_struct_members(&self) -> Vec<StructMember> {
        self.getter_return_parameters()
            .unwrap_or_default()
            .iter()
            .filter_map(|parameter| {
                match Definition::try_create(parameter.definition_id()?, &self.semantic) {
                    Some(Definition::StructMember(member)) => Some(member),
                    _ => None,
                }
            })
            .collect()
    }
}
