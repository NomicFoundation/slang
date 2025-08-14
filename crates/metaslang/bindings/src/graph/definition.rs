use std::fmt::{self, Debug, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_cst::nodes::NodeId;

use super::{BindingGraph, BindingLocation, Reference};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

/// Represents a definition in the binding graph.
#[derive(Clone)]
pub struct Definition<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Definition<KT> {
    /// Returns a unique numerical identifier of the definition.
    /// It is only valid for the lifetime of the binding graph.
    /// It can change between multiple graphs, even for the same source code input.
    pub fn id(&self) -> NodeId {
        self.internal_get_definiens_cursor().node().id()
    }

    /// Returns the location of the definition's name.
    /// For `contract X {}`, that is the location of the `X` `Identifier` node.
    pub fn name_location(&self) -> BindingLocation<KT> {
        match self.internal_get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.internal_get_cursor().to_owned())
            }
        }
    }

    /// Returns the location of the definition's definiens.
    /// For `contract X {}`, that is the location of the parent `ContractDefinition` node.
    pub fn definiens_location(&self) -> BindingLocation<KT> {
        match self.internal_get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.internal_get_definiens_cursor().to_owned())
            }
        }
    }

    /// Returns a list of all references that bind to this definition.
    pub fn references(&self) -> Vec<Reference<KT>> {
        self.owner.resolve_definition(self.handle)
    }

    fn internal_get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_cursor(self.handle)
            .expect("Definition handle is valid")
    }

    fn internal_get_definiens_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_definiens_cursor(self.handle)
            .expect("Definition handle is valid")
    }

    fn internal_get_file(&self) -> FileDescriptor {
        self.owner
            .graph
            .get_file_descriptor(self.handle)
            .expect("Definition to have a valid file descriptor")
    }
}

#[cfg(feature = "__private_testing_utils")]
#[allow(missing_docs)]
impl<KT: KindTypes + 'static> Definition<KT> {
    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.internal_get_cursor()
    }

    pub fn get_definiens_cursor(&self) -> &Cursor<KT> {
        self.internal_get_definiens_cursor()
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.internal_get_file()
    }
}

impl<KT: KindTypes + 'static> Display for Definition<KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "definition {}",
            DisplayCursor {
                cursor: self.internal_get_cursor(),
                file: self.internal_get_file()
            }
        )
    }
}

impl<KT: KindTypes + 'static> Debug for Definition<KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<KT: KindTypes + 'static> PartialEq for Definition<KT> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.owner, &other.owner) && self.handle == other.handle
    }
}

impl<KT: KindTypes + 'static> Eq for Definition<KT> {}
