use slang_solidity_v2_semantic::binder;
pub use slang_solidity_v2_semantic::binder::Recursion;

use super::super::StructDefinitionStruct;

impl StructDefinitionStruct {
    /// The reach through which this struct meets a cycle of the unit's struct
    /// graph, or `None` when it meets none. Every cycle carries a struct
    /// reached through a dynamic array, a mapping or a function type, so
    /// keeping only [`Recursion::Indirect`] gives solc's `recursive` set
    /// exactly.
    pub fn recursion(&self) -> Option<Recursion> {
        let Some(binder::Definition::Struct(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("definition is not a struct");
        };
        definition.recursion
    }

    pub fn is_recursive(&self) -> bool {
        self.recursion().is_some()
    }
}
