use std::fmt::{self, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_cst::nodes::NodeId;

use super::{BindingGraph, BindingLocation, Definition};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

/// Represents a reference to a node in the binding graph.
#[derive(Clone)]
pub struct Reference<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Reference<KT> {
    /// Returns a unique numerical identifier of the reference.
    /// It is only valid for the lifetime of the binding graph.
    /// It can change between multiple graphs, even for the same source code input.
    pub fn id(&self) -> NodeId {
        self.internal_get_cursor().node().id()
    }

    /// Returns the location of the reference.
    /// For `new X()`, that is the location of the `X` `Identifier` node.
    pub fn location(&self) -> BindingLocation<KT> {
        match self.internal_get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.internal_get_cursor().to_owned())
            }
        }
    }

    /// Returns a list of all definitions related to this reference.
    /// Most references have a single definition, but some have multiple, such as when a symbol
    /// is imported from another file, and renamed (re-defined) in the current file.
    pub fn definitions(&self) -> Vec<Definition<KT>> {
        self.owner.resolve_reference(self.handle)
    }

    fn internal_get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_cursor(self.handle)
            .expect("Reference handle is valid")
    }

    fn internal_get_file(&self) -> FileDescriptor {
        self.owner
            .graph
            .get_file_descriptor(self.handle)
            .expect("Reference to have a valid file descriptor")
    }
}

#[cfg(feature = "__private_testing_utils")]
#[allow(missing_docs)]
impl<KT: KindTypes + 'static> Reference<KT> {
    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.internal_get_cursor()
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.internal_get_file()
    }
}

impl<KT: KindTypes + 'static> Display for Reference<KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "reference {}",
            DisplayCursor {
                cursor: self.internal_get_cursor(),
                file: self.internal_get_file()
            }
        )
    }
}

impl<KT: KindTypes + 'static> PartialEq for Reference<KT> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.owner, &other.owner) && self.handle == other.handle
    }
}
