use super::super::{Definition, FunctionDefinitionStruct};

impl FunctionDefinitionStruct {
    /// Returns the contract / library / interface this function is a member of,
    /// or `None` for a file-level (free) function. Resolved through the binder's
    /// scope chain — the function's own scope is keyed by its node id and its
    /// parent scope belongs to the enclosing definition.
    pub fn enclosing_definition(&self) -> Option<Definition> {
        let enclosing_node_id = self
            .semantic
            .binder()
            .enclosing_definition_node_id(self.ir_node.id())?;
        Definition::try_create(enclosing_node_id, &self.semantic)
    }
}
