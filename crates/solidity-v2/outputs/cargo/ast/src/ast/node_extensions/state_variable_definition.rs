use slang_solidity_v2_semantic::binder;

use super::super::{StateVariableDefinitionStruct, Type};

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
}
