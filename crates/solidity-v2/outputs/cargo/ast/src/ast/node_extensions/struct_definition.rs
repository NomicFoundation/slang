use slang_solidity_v2_semantic::binder;

use super::super::StructDefinitionStruct;

impl StructDefinitionStruct {
    /// Whether a cycle of struct members is reachable from this struct through
    /// nested structs, arrays and mappings, or this struct is on a cycle closed
    /// by a function type among the structs that reach none.
    pub fn is_recursive(&self) -> bool {
        let Some(binder::Definition::Struct(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("struct node without a struct definition");
        };
        definition.is_recursive
    }
}
