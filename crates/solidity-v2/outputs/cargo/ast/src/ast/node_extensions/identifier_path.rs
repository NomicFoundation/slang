use super::super::nodes::IdentifierPathStruct;
use crate::ast::Definition;

impl IdentifierPathStruct {
    pub fn name(&self) -> String {
        self.ir_nodes
            .iter()
            .map(|ir_node| ir_node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    }

    /// Attempts to resolve the identifier path to a definition, following
    /// symbol aliases (import deconstructions).
    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_definition_id(ir_node.id())?;
        Definition::try_create(definition_id, &self.semantic)
    }

    /// Attempts to resolve the identifier path to an immediate definition,
    /// without following symbol aliases, possibly returning import
    /// deconstruction symbols. If the path refers to a direct definition, this
    /// is equivalent to `resolve_to_definition`.
    pub fn resolve_to_immediate_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_immediate_definition_id(ir_node.id())?;
        Definition::try_create(definition_id, &self.semantic)
    }
}
