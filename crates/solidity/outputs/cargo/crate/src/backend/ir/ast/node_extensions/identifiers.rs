use std::rc::Rc;

use super::super::IdentifierPathStruct;
use super::Definition;
use crate::backend::SemanticAnalysis;
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub type Identifier = Rc<IdentifierStruct>;

pub(crate) fn create_identifier(
    ir_node: &Rc<TerminalNode>,
    semantic: &Rc<SemanticAnalysis>,
) -> Identifier {
    assert!(ir_node.kind == TerminalKind::Identifier);
    Rc::new(IdentifierStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

pub struct IdentifierStruct {
    ir_node: Rc<TerminalNode>,
    semantic: Rc<SemanticAnalysis>,
}

impl IdentifierStruct {
    pub fn unparse(&self) -> String {
        self.ir_node.unparse()
    }

    pub fn is_reference(&self) -> bool {
        self.semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    /// Attempts to resolve the identifier to a definition, following symbol
    /// aliases (import deconstructions). This only makes sense if
    /// `is_reference()` is true.
    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_definition_id(self.ir_node.id())?;
        Some(Definition::create(definition_id, &self.semantic))
    }

    // Attempts to resolve the identifier to an immediate definition, without
    // following symbol aliases, possibly returning import deconstruction
    // symbols. If the symbol is directly defined, this is equivalent to
    // `resolve_to_definition`. This only makes sense if `is_reference()` is
    // true.
    pub fn resolve_to_immediate_definition(&self) -> Option<Definition> {
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_immediate_definition_id(self.ir_node.id())?;
        Some(Definition::create(definition_id, &self.semantic))
    }

    /// Returns `true` if the identifier itself is a definition (eg. an enum member)
    pub fn is_definition(&self) -> bool {
        self.semantic
            .binder()
            .find_definition_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    // only makes sense if `is_definition()` is true
    pub fn references(&self) -> Vec<Reference> {
        self.semantic.references_binding_to(self.ir_node.id())
    }
}

pub type YulIdentifierStruct = IdentifierStruct;
pub type YulIdentifier = Rc<YulIdentifierStruct>;

pub(crate) fn create_yul_identifier(
    ir_node: &Rc<TerminalNode>,
    semantic: &Rc<SemanticAnalysis>,
) -> YulIdentifier {
    assert!(ir_node.kind == TerminalKind::YulIdentifier);
    Rc::new(YulIdentifierStruct {
        ir_node: Rc::clone(ir_node),
        semantic: Rc::clone(semantic),
    })
}

pub enum Reference {
    Identifier(Identifier),
    YulIdentifier(YulIdentifier),
}

impl Reference {
    pub(crate) fn try_create(
        node: &Rc<TerminalNode>,
        semantic: &Rc<SemanticAnalysis>,
    ) -> Option<Self> {
        // ensure the terminal node is actually functioning as a reference
        semantic
            .binder()
            .find_reference_by_identifier_node_id(node.id())?;

        match node.kind {
            TerminalKind::Identifier => {
                Some(Reference::Identifier(create_identifier(node, semantic)))
            }
            TerminalKind::YulIdentifier => Some(Reference::YulIdentifier(create_yul_identifier(
                node, semantic,
            ))),
            _ => None,
        }
    }

    pub fn unparse(&self) -> String {
        match self {
            Reference::Identifier(identifier) | Reference::YulIdentifier(identifier) => {
                identifier.unparse()
            }
        }
    }

    pub fn resolve_to_definition(&self) -> Option<Definition> {
        match self {
            Reference::Identifier(identifier) | Reference::YulIdentifier(identifier) => {
                identifier.resolve_to_definition()
            }
        }
    }

    pub fn resolve_to_immediate_definition(&self) -> Option<Definition> {
        match self {
            Reference::Identifier(identifier) | Reference::YulIdentifier(identifier) => {
                identifier.resolve_to_immediate_definition()
            }
        }
    }
}

impl SemanticAnalysis {
    pub(crate) fn references_binding_to(self: &Rc<Self>, node_id: NodeId) -> Vec<Reference> {
        let node_ids = self.binder().get_references_by_definition_id(node_id);

        node_ids
            .iter()
            .filter_map(|node_id| {
                self.binder()
                    .find_reference_by_identifier_node_id(*node_id)
                    .and_then(|reference| Reference::try_create(&reference.identifier, self))
            })
            .collect()
    }

    fn resolve_reference_identifier_to_definition_id(&self, node_id: NodeId) -> Option<NodeId> {
        let reference = self
            .binder()
            .find_reference_by_identifier_node_id(node_id)?;
        self.binder()
            .follow_symbol_aliases(&reference.resolution)
            .as_definition_id()
    }

    fn resolve_reference_identifier_to_immediate_definition_id(
        &self,
        node_id: NodeId,
    ) -> Option<NodeId> {
        let reference = self
            .binder()
            .find_reference_by_identifier_node_id(node_id)?;
        reference.resolution.as_definition_id()
    }
}

impl IdentifierPathStruct {
    pub fn unparse(&self) -> String {
        self.ir_nodes
            .iter()
            .map(|ir_node| ir_node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    }

    // Attempts to resolve the identifier path to a definition, following symbol
    // aliases (import deconstructions)
    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_definition_id(ir_node.id())?;
        Some(Definition::create(definition_id, &self.semantic))
    }

    // Attempts to resolve the identifier path to an immediate definition,
    // without following symbol aliases, possibly returning import
    // deconstruction symbols. If the symbol is directly defined, this is
    // equivalent to `resolve_to_definition`. This only makes sense if
    // `is_reference()` is true.
    pub fn resolve_to_immediate_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let definition_id = self
            .semantic
            .resolve_reference_identifier_to_immediate_definition_id(ir_node.id())?;
        Some(Definition::create(definition_id, &self.semantic))
    }
}
