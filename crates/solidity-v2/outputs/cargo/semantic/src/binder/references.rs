use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use smallvec::smallvec;

use super::DefinitionIds;
use crate::built_ins::InternalBuiltIn;

//////////////////////////////////////////////////////////////////////////////
// References

#[derive(Debug)]
pub struct Reference {
    pub identifier: ir::Identifier,
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
    Ambiguous(Box<[NodeId]>),
    /// The symbol refers to a Solidity built-in of some kind. The possible
    /// variants are encoded in an enum and the behaviour of each is encoded in
    /// the `built_ins.rs` module.
    BuiltIn(InternalBuiltIn),
}

impl Reference {
    pub fn node_id(&self) -> NodeId {
        self.identifier.id()
    }

    pub(crate) fn new(identifier: ir::Identifier, resolution: Resolution) -> Self {
        Self {
            identifier,
            resolution,
        }
    }
}

impl Resolution {
    pub(crate) fn as_definition_id(&self) -> Option<NodeId> {
        if let Resolution::Definition(definition_id) = self {
            Some(*definition_id)
        } else {
            None
        }
    }

    pub(crate) fn get_definition_ids(&self) -> DefinitionIds {
        match self {
            Resolution::Definition(id) => smallvec![*id],
            Resolution::Ambiguous(ids) => DefinitionIds::from_slice(ids),
            _ => DefinitionIds::new(),
        }
    }

    #[must_use]
    pub(crate) fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if self == Self::Unresolved { f() } else { self }
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
            // Takes over the vector's buffer rather than copying it.
            _ => Resolution::Ambiguous(value.into_boxed_slice()),
        }
    }
}

impl From<DefinitionIds> for Resolution {
    fn from(mut value: DefinitionIds) -> Self {
        match value.len() {
            0 => Resolution::Unresolved,
            1 => Resolution::Definition(value.swap_remove(0)),
            // Only an ambiguous result reaches the heap, and it is the rare
            // case: a name shared by an overload set or by declarations in
            // several imported files. Keeping it boxed rather than inline
            // keeps `Resolution` (and so every `Reference` the binder stores)
            // down to the width of a single pointer plus its length.
            _ => Resolution::Ambiguous(value.into_vec().into_boxed_slice()),
        }
    }
}

/// Builds a resolution from the definitions a name refers to, borrowing them.
/// The single-definition case, which is by far the common one, stays a slice
/// read rather than a copy, so this is preferred over the owning conversions
/// wherever the definitions are only on loan.
impl From<&[NodeId]> for Resolution {
    fn from(value: &[NodeId]) -> Self {
        match value {
            [] => Resolution::Unresolved,
            &[definition_id] => Resolution::Definition(definition_id),
            _ => Resolution::Ambiguous(value.into()),
        }
    }
}

impl From<Option<InternalBuiltIn>> for Resolution {
    fn from(value: Option<InternalBuiltIn>) -> Self {
        if let Some(built_in) = value {
            Self::BuiltIn(built_in)
        } else {
            Self::Unresolved
        }
    }
}
