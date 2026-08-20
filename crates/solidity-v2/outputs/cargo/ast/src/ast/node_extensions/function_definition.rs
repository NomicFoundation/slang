use slang_solidity_v2_semantic::binder;

use super::super::{FunctionDefinitionStruct, Type};

impl FunctionDefinitionStruct {
    /// Returns the type this function is dispatched through — external
    /// visibility, with `calldata` locations changed to `memory` — or `None`
    /// when nothing selects on it: an internal or private function, a modifier,
    /// or a constructor, fallback or receive.
    pub fn externalized_type(&self) -> Option<Type> {
        let binder::Definition::Function(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            return None;
        };
        Some(Type::create(
            definition.externalized_type_id?,
            &self.semantic,
        ))
    }
}
