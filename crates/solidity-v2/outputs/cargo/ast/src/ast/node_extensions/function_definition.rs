use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::{binder, types};

use super::super::{FunctionDefinitionStruct, Type};

impl FunctionDefinitionStruct {
    /// Returns the type this function is dispatched through — external
    /// visibility, with `calldata` locations changed to `memory` — or `None`
    /// when nothing selects on it: an internal or private function, a modifier,
    /// or a constructor, fallback or receive.
    pub fn externalized_type(&self) -> Option<Type> {
        let type_id = externalized_type_id_of_function(&self.semantic, self.ir_node.id())?;
        Some(Type::create(type_id, &self.semantic))
    }
}

/// The externalized type of the function declared at `definition_id`. Only a
/// named `Regular` function has a binder definition, and p3 externalizes each
/// externally visible one.
pub(crate) fn externalized_type_id_of_function(
    semantic: &SemanticContext,
    definition_id: NodeId,
) -> Option<types::TypeId> {
    let Some(binder::Definition::Function(_)) =
        semantic.binder().find_definition_by_id(definition_id)
    else {
        return None;
    };
    let type_id = semantic.binder().node_typing(definition_id).as_type_id()?;
    let types::Type::Function(function_type) = semantic.types().get_type_by_id(type_id) else {
        return None;
    };
    if !function_type.is_externally_visible() {
        return None;
    }
    semantic.types().externalized_function_type_id(type_id)
}
