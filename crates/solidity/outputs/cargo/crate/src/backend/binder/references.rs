use std::rc::Rc;

use crate::backend::built_ins::BuiltIn;
use crate::cst::{NodeId, TerminalNode};

//////////////////////////////////////////////////////////////////////////////
// References

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    Unresolved,
    Definition(NodeId),
    Ambiguous(Vec<NodeId>),
    BuiltIn(BuiltIn),
}

impl Resolution {
    pub fn as_definition_id(&self) -> Option<NodeId> {
        if let Resolution::Definition(definition_id) = self {
            Some(*definition_id)
        } else {
            None
        }
    }

    pub(crate) fn get_definition_ids(&self) -> Vec<NodeId> {
        match self {
            Resolution::Definition(id) => vec![*id],
            Resolution::Ambiguous(ids) => ids.clone(),
            _ => Vec::new(),
        }
    }

    #[must_use]
    pub fn non_ambiguous(self) -> Self {
        if matches!(self, Self::Ambiguous(_)) {
            Self::Unresolved
        } else {
            self
        }
    }

    #[must_use]
    pub fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if self == Self::Unresolved {
            f()
        } else {
            self
        }
    }
}

impl From<Option<NodeId>> for Resolution {
    fn from(value: Option<NodeId>) -> Self {
        if let Some(definition_id) = value {
            Self::Definition(definition_id)
        } else {
            Self::Unresolved
        }
    }
}

impl From<Option<&NodeId>> for Resolution {
    fn from(value: Option<&NodeId>) -> Self {
        if let Some(definition_id) = value {
            Self::Definition(*definition_id)
        } else {
            Self::Unresolved
        }
    }
}

impl From<Vec<NodeId>> for Resolution {
    fn from(mut value: Vec<NodeId>) -> Self {
        match value.len() {
            0 => Resolution::Unresolved,
            1 => Resolution::Definition(value.swap_remove(0)),
            _ => Resolution::Ambiguous(value),
        }
    }
}

impl From<Option<BuiltIn>> for Resolution {
    fn from(value: Option<BuiltIn>) -> Self {
        if let Some(built_in) = value {
            Self::BuiltIn(built_in)
        } else {
            Self::Unresolved
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub identifier: Rc<TerminalNode>,
    pub resolution: Resolution,
}

impl Reference {
    pub(crate) fn node_id(&self) -> NodeId {
        self.identifier.id()
    }

    pub(crate) fn new(identifier: Rc<TerminalNode>, resolution: Resolution) -> Self {
        Self {
            identifier,
            resolution,
        }
    }
}
