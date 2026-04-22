use slang_solidity_v2_common::built_ins::BuiltIn;
use slang_solidity_v2_ir::ir::NodeId;
use slang_solidity_v2_semantic::binder;

use super::super::nodes::{IdentifierPathStruct, IdentifierStruct};
use crate::ast::references::references_binding_to_definition;
use crate::ast::{Definition, Reference};

impl IdentifierStruct {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> String {
        self.ir_node.unparse().to_string()
    }

    pub fn is_reference(&self) -> bool {
        self.semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    /// Attempts to resolve the identifier to a definition, following symbol
    /// aliases (import deconstructions). Returns `None` if the identifier is
    /// not acting as a reference, or the reference cannot be resolved.
    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_definition_id(self.ir_node.id())?;
        Definition::try_create(definition_id, &self.semantic)
    }

    /// Attempts to resolve the identifier to an immediate definition, without
    /// following symbol aliases, possibly returning import deconstruction
    /// symbols. If the identifier refers to a direct definition, this is
    /// equivalent to `resolve_to_definition()`. Returns `None` if the
    /// identifier is not acting as a reference, or the reference cannot be
    /// resolved.
    pub fn resolve_to_immediate_definition(&self) -> Option<Definition> {
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_immediate_definition_id(self.ir_node.id())?;
        Definition::try_create(definition_id, &self.semantic)
    }

    /// If this identifier resolves to a built-in (e.g. `msg`, `block`,
    /// `tx`), returns the corresponding [`BuiltIn`] variant.
    pub fn resolve_to_built_in(&self) -> Option<BuiltIn> {
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())?;
        match &reference.resolution {
            binder::Resolution::BuiltIn(built_in) => Some(built_in.to_public()),
            _ => None,
        }
    }

    /// Returns `true` if the identifier itself is a definition (eg. an enum member)
    pub fn is_definition(&self) -> bool {
        self.as_definition().is_some()
    }

    /// Returns the `Definition` corresponding to this identifier. Panics if the
    /// identifier is not a definition by itself, ie. this can only be called
    /// safely if `is_definition()` returns `true`.
    pub fn as_definition(&self) -> Option<Definition> {
        Definition::try_create(self.ir_node.id(), &self.semantic)
    }

    /// Returns `true` if the identifier is a definition itself, or is the name
    /// identifier of a definition
    pub fn is_name_of_definition(&self) -> bool {
        self.semantic
            .binder()
            .find_definition_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    // only makes sense if `is_definition()` is true
    pub fn references(&self) -> Vec<Reference> {
        references_binding_to_definition(self.ir_node.id(), &self.semantic)
    }
}

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
