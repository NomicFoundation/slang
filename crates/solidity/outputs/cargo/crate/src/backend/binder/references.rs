use std::rc::Rc;

use crate::backend::built_ins::BuiltIn;
use crate::cst::{NodeId, SyntaxNode};

//////////////////////////////////////////////////////////////////////////////
// References

#[derive(Debug)]
pub struct Reference {
    pub identifier: Rc<SyntaxNode>,
    pub resolution: Resolution,
}

/// `Resolution` represents the result of a lookup of a reference in the context
/// where that reference occurs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    /// The identifier was not found.
    Unresolved,
    /// A single definition identified by a `NodeId`.
    Definition(NodeId),
    /// When the lookup returns multiple possible definitions (eg. multiple
    /// function overloads, or virtual methods in a contract hierarchy), it's
    /// usually not possible to determine which of the found definitions should
    /// apply, as more information is needed (eg. the types of arguments when
    /// the reference is used in a function call, to select the appropriate
    /// overload).
    Ambiguous(Vec<NodeId>),
    /// The symbol refers to a Solidity built-in of some kind. The possible
    /// variants are encoded in an enum and the behaviour of each is encoded in
    /// the `built_ins.rs` module.
    BuiltIn(BuiltIn),
}

impl Reference {
    pub fn node_id(&self) -> NodeId {
        self.identifier.id()
    }

    pub(crate) fn new(identifier: Rc<SyntaxNode>, resolution: Resolution) -> Self {
        Self {
            identifier,
            resolution,
        }
    }
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
