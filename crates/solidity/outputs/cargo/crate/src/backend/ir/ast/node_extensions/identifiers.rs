use std::rc::Rc;

use super::{Definition, DefinitionStruct, IdentifierPathStruct};
use crate::backend::SemanticAnalysis;
use crate::cst::{TerminalKind, TerminalNode};

pub type Identifier = Rc<IdentifierStruct>;

pub struct IdentifierStruct {
    ir_node: Rc<TerminalNode>,
    semantic: Rc<SemanticAnalysis>,
}

impl IdentifierStruct {
    pub(crate) fn create(ir_node: &Rc<TerminalNode>, semantic: &Rc<SemanticAnalysis>) -> Self {
        assert!(ir_node.kind == TerminalKind::Identifier);
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn unparse(&self) -> String {
        self.ir_node.unparse()
    }

    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())?;
        let definition_id = reference.resolution.as_definition_id()?;
        Some(Rc::new(DefinitionStruct::create(
            definition_id,
            &self.semantic,
        )))
    }

    pub fn is_definition(&self) -> bool {
        self.semantic
            .binder()
            .find_definition_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    pub fn is_reference(&self) -> bool {
        self.semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }
}

pub type YulIdentifierStruct = IdentifierStruct;
pub type YulIdentifier = Rc<YulIdentifierStruct>;

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
        Some(Rc::new(DefinitionStruct::create(
            definition_id,
            &self.semantic,
        )))
    }
}
