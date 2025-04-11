use std::fmt::{self, Debug, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
use metaslang_cst::nodes::NodeId;

use super::{BindingGraph, BindingLocation, Reference};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

#[derive(Clone)]
pub struct Definition<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Definition<KT> {
    pub fn id(&self) -> NodeId {
        self.get_cursor().node().id()
    }

    pub fn name_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn definiens_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::BuiltIns(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_definiens_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_cursor(self.handle)
            .expect("Definition handle is valid")
    }

    pub fn get_definiens_cursor(&self) -> &Cursor<KT> {
        self.owner
            .graph
            .get_definiens_cursor(self.handle)
            .expect("Definition handle is valid")
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner
            .graph
            .get_file_descriptor(self.handle)
            .expect("Definition to have a valid file descriptor")
    }

    pub fn references(&self) -> Vec<Reference<KT>> {
        self.owner.resolve_definition(self.handle)
    }
}

impl<KT: KindTypes + 'static> Display for Definition<KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "definition {}",
            DisplayCursor {
                cursor: self.get_cursor(),
                file: self.get_file()
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
