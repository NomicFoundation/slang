use std::rc::Rc;

use super::super::IdentifierPathStruct;
use super::Definition;
use crate::backend::SemanticAnalysis;
use crate::cst::{NodeId, TerminalKind, TerminalNode, TextIndex};

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
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> String {
        self.ir_node.unparse()
    }

    pub fn is_reference(&self) -> bool {
        self.semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    // only makes sense if `is_reference()` is true
    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())?;
        let definition_id = reference.resolution.as_definition_id()?;
        Definition::try_create(definition_id, &self.semantic)
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
        self.semantic.references_binding_to(self.ir_node.id())
    }

    pub fn text_offset(&self) -> TextIndex {
        self.semantic
            .get_text_offset_by_node_id(self.ir_node.id())
            .unwrap()
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
}

impl IdentifierPathStruct {
    pub fn unparse(&self) -> String {
        self.ir_nodes
            .iter()
            .map(|ir_node| ir_node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(ir_node.id())?;
        let definition_id = reference.resolution.as_definition_id()?;
        Definition::try_create(definition_id, &self.semantic)
    }
}
