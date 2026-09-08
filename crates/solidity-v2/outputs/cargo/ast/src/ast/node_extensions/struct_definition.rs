use slang_solidity_v2_semantic::binder;

use super::super::StructDefinitionStruct;

impl StructDefinitionStruct {
    /// Whether the struct is recursive: a cycle is reachable from it through
    /// dynamic arrays, mappings, fixed-size arrays or nested structs, which
    /// solc's `recursive` annotation marks, or it lies on a cycle closed by a
    /// function type that passes through no such struct. Every cycle of a
    /// unit's struct graph then carries a recursive struct reached through a
    /// dynamic array, a mapping or a function type.
    pub fn is_recursive(&self) -> bool {
        let Some(binder::Definition::Struct(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("definition is not a struct");
        };
        definition.is_recursive
    }
}
