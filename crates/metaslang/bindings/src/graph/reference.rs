use std::fmt::{self, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_cst::nodes::NodeId;

use super::{BindingGraph, BindingLocation, Definition};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

#[derive(Clone)]
pub struct Reference<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Reference<KT> {
    pub fn id(&self) -> NodeId {
        self.get_cursor().node().id()
    }

    pub fn location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_cursor(self.handle)
            .expect("Reference handle is valid")
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner
            .graph
            .get_file_descriptor(self.handle)
            .expect("Reference to have a valid file descriptor")
    }

    pub fn definitions(&self) -> Vec<Definition<KT>> {
        self.owner.resolve_reference(self.handle)
    }
}

impl<KT: KindTypes + 'static> Display for Reference<KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "reference {}",
            DisplayCursor {
                cursor: self.get_cursor(),
                file: self.get_file()
            }
        )
    }
}

impl<KT: KindTypes + 'static> PartialEq for Reference<KT> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.owner, &other.owner) && self.handle == other.handle
    }
}
