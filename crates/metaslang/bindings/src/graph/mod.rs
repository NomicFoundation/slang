mod definition;
mod location;
mod reference;
mod resolver;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub use definition::Definition;
pub use location::{BindingLocation, BuiltInLocation, UserFileLocation};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
pub use reference::Reference;

use crate::builder::{BindingInfo, FileDescriptor, FileHandle, GraphHandle};
use resolver::Resolver;

pub struct BindingGraph<KT: KindTypes + 'static> {
    info: BindingInfo<KT>,
    resolver: Rc<RefCell<Resolver>>,
}

impl<KT: KindTypes + 'static> BindingGraph<KT> {
    pub(crate) fn build(builder: BindingInfo<KT>) -> Rc<Self> {
        let mut resolver = Resolver::new(&builder);
        resolver.resolve(&builder);

        Rc::new(Self {
            info: builder,
            resolver: Rc::new(RefCell::new(resolver)),
        })
    }

    pub fn all_definitions(self: &Rc<Self>) -> impl Iterator<Item = Definition<KT>> + '_ {
        self.info
            .stack_graph
            .iter_nodes()
            .filter(|handle| self.info.stack_graph[*handle].is_definition())
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle,
            })
    }

    fn to_definition(self: &Rc<Self>, handle: GraphHandle) -> Option<Definition<KT>> {
        if self.info.stack_graph[handle].is_definition() {
            Some(Definition {
                owner: Rc::clone(self),
                handle,
            })
        } else {
            None
        }
    }

    pub fn all_references(self: &Rc<Self>) -> impl Iterator<Item = Reference<KT>> + '_ {
        self.info
            .stack_graph
            .iter_nodes()
            .filter(|handle| self.info.stack_graph[*handle].is_reference())
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle,
            })
    }

    pub fn definition_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Definition<KT>> {
        let cursor_id = cursor.node().id();
        self.info
            .cursor_to_definitions
            .get(&cursor_id)
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle: *handle,
            })
    }

    pub fn reference_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Reference<KT>> {
        let cursor_id = cursor.node().id();
        self.info
            .cursor_to_references
            .get(&cursor_id)
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle: *handle,
            })
    }

    fn get_file(&self, handle: FileHandle) -> FileDescriptor {
        FileDescriptor::from(self.info.stack_graph[handle].name())
    }
}

struct DisplayCursor<'a, KT: KindTypes + 'static> {
    cursor: &'a Cursor<KT>,
    file: FileDescriptor,
}

impl<'a, KT: KindTypes + 'static> fmt::Display for DisplayCursor<'a, KT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.cursor.text_offset();
        write!(
            f,
            "`{}` at {}:{}:{}",
            self.cursor.node().unparse(),
            self.file.get_path(),
            offset.line + 1,
            offset.column + 1,
        )
    }
}
