use std::fmt::{self, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;

use super::{BindingGraph, BindingLocation, Definition};
use crate::builder::{FileDescriptor, GraphHandle};
use crate::graph::DisplayCursor;

#[derive(Clone)]
pub struct Reference<KT: KindTypes + 'static> {
    pub(crate) owner: Rc<BindingGraph<KT>>,
    pub(crate) handle: GraphHandle,
}

impl<KT: KindTypes + 'static> Reference<KT> {
    pub fn id(&self) -> usize {
        self.get_cursor().node().id()
    }

    pub fn location(&self) -> BindingLocation<KT> {
        match self.get_file() {
            FileDescriptor::System(_) => BindingLocation::built_in(),
            FileDescriptor::User(file_id) => {
                BindingLocation::user_file(file_id, self.get_cursor().to_owned())
            }
        }
    }

    pub fn get_cursor(&self) -> &Cursor<KT> {
        &self
            .owner
            .references
            .get(&self.handle)
            .expect("Reference handle is valid")
            .cursor
    }

    pub fn get_file(&self) -> FileDescriptor {
        self.owner
            .get_file(
                self.owner
                    .references
                    .get(&self.handle)
                    .expect("Reference handle is valid")
                    .file,
            )
            .expect("Reference does not have a valid file descriptor")
    }

    pub fn definitions(&self) -> Vec<Definition<KT>> {
        let resolver = self.owner.resolver.borrow();
        resolver.references[&self.handle]
            .iter()
            .map(|handle| {
                self.owner
                    .to_definition(*handle)
                    .expect("Resolved reference handle to be a definition")
            })
            .collect()
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
