use slang_solidity_v2_semantic::binder;

use super::super::StructDefinitionStruct;

impl StructDefinitionStruct {
    /// Whether a cycle is reachable from this struct through struct members,
    /// arrays and mapping values, excluding function parameter and return types.
    /// Includes structs that reach a cycle without belonging to it.
    ///
    /// This is true for valid structs too, such as `struct S { S[] children; }`:
    /// only cycles held by value make a struct infinitely sized.
    pub fn is_recursive(&self) -> bool {
        let Some(binder::Definition::Struct(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("struct node without a struct definition");
        };
        definition.recursive >= Some(binder::RecursionKind::ByReference)
    }

    /// Whether a cycle is reachable from this struct through struct members,
    /// arrays and mapping values, including function parameter and return types.
    /// Includes structs that reach a cycle without belonging to it, so this is
    /// always true when [`Self::is_recursive`] is true.
    ///
    /// This describes recursion when expanding type descriptions, not the size
    /// of runtime objects.
    pub fn has_recursive_type_graph(&self) -> bool {
        let Some(binder::Definition::Struct(definition)) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())
        else {
            unreachable!("struct node without a struct definition");
        };
        definition.recursive >= Some(binder::RecursionKind::ByFunctionType)
    }
}
