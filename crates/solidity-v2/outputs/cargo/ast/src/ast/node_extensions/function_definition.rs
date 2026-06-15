use super::super::nodes::FunctionDefinitionStruct;
use crate::ast::Definition;

impl FunctionDefinitionStruct {
    /// The contract, library, or interface this function is a member of, or
    /// `None` for a file-level (free) function.
    pub fn enclosing_definition(&self) -> Option<Definition> {
        let enclosing = self
            .semantic
            .binder()
            .enclosing_scope_node_id(self.ir_node.id())?;
        Definition::try_create(enclosing, &self.semantic)
    }
}
