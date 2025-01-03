use std::fmt::{self, Debug, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;

use super::{BindingGraph, BindingLocation};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

#[derive(Clone)]
pub struct Definition<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Definition<KT> {
    pub fn id(&self) -> usize {
        self.get_cursor().node().id()
    }

    pub fn name_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn definiens_location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_definiens_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        &self
            .owner
            .definitions
            .get(&self.handle)
            .expect("Definition handle is valid")
            .cursor
    }

    pub fn get_definiens_cursor(&self) -> &Cursor<KT> {
        &self
            .owner
            .definitions
            .get(&self.handle)
            .expect("Definition handle is valid")
            .definiens
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner
            .get_file(
                self.owner
                    .definitions
                    .get(&self.handle)
                    .expect("Definition handle is valid")
                    .file,
            )
            .expect("Definition does not have a valid file descriptor")
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
