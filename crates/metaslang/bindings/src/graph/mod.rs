mod definition;
mod location;
mod reference;
mod resolver;

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::rc::Rc;

pub use definition::Definition;
pub use location::{BindingLocation, BuiltInLocation, UserFileLocation};
use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;
pub use reference::Reference;

use resolver::Resolver;
use crate::builder::{CursorID, FileDescriptor, FileHandle, GraphHandle};
use crate::BindingGraphBuilder;

struct DefinitionInfo<KT: KindTypes + 'static> {
    file: FileHandle,
    cursor: Cursor<KT>,
    definiens: Cursor<KT>,
}

struct ReferenceInfo<KT: KindTypes + 'static> {
    file: FileHandle,
    cursor: Cursor<KT>,
}

pub struct BindingGraph<KT: KindTypes + 'static> {
    files: HashMap<FileHandle, FileDescriptor>,
    definitions: BTreeMap<GraphHandle, DefinitionInfo<KT>>,
    references: BTreeMap<GraphHandle, ReferenceInfo<KT>>,
    cursor_to_definitions: HashMap<CursorID, GraphHandle>,
    cursor_to_references: HashMap<CursorID, GraphHandle>,
    resolved_references: HashMap<GraphHandle, Vec<GraphHandle>>,
}

impl<KT: KindTypes + 'static> BindingGraph<KT> {
    pub(crate) fn build(
        builder: BindingGraphBuilder<KT>,
    ) -> Rc<Self> {
        let resolver = Resolver::new(&builder);
        let resolved_references = resolver.resolve(&builder);

        let mut files = HashMap::new();
        for handle in builder.stack_graph.iter_files() {
            files.insert(
                handle,
                FileDescriptor::from(builder.stack_graph[handle].name()),
            );
        }
        let mut definitions = BTreeMap::new();
        let mut references = BTreeMap::new();
        for handle in builder.stack_graph.iter_nodes() {
            let graph_node = &builder.stack_graph[handle];
            let Some(file) = graph_node.file() else {
                continue;
            };
            if graph_node.is_definition() {
                let definition_info = &builder.definitions_info[&handle];
                let cursor = definition_info.cursor.clone();
                let definiens = definition_info.definiens.clone();
                definitions.insert(
                    handle,
                    DefinitionInfo {
                        file,
                        cursor,
                        definiens,
                    },
                );
            } else if graph_node.is_reference() {
                let reference_info = &builder.references_info[&handle];
                let cursor = reference_info.cursor.clone();
                references.insert(handle, ReferenceInfo { file, cursor });
            }
        }

        Rc::new(Self {
            files,
            definitions,
            references,
            cursor_to_definitions: builder.cursor_to_definitions,
            cursor_to_references: builder.cursor_to_references,
            resolved_references,
        })
    }

    pub fn all_definitions(self: &Rc<Self>) -> impl Iterator<Item = Definition<KT>> + '_ {
        self.definitions.keys().map(|handle| Definition {
            owner: Rc::clone(self),
            handle: *handle,
        })
    }

    fn to_definition(self: &Rc<Self>, handle: GraphHandle) -> Option<Definition<KT>> {
        if self.definitions.contains_key(&handle) {
            Some(Definition {
                owner: Rc::clone(self),
                handle,
            })
        } else {
            None
        }
    }

    pub fn all_references(self: &Rc<Self>) -> impl Iterator<Item = Reference<KT>> + '_ {
        self.references.keys().map(|handle| Reference {
            owner: Rc::clone(self),
            handle: *handle,
        })
    }

    pub fn definition_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Definition<KT>> {
        let cursor_id = cursor.node().id();
        self.cursor_to_definitions
            .get(&cursor_id)
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle: *handle,
            })
    }

    pub fn reference_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Reference<KT>> {
        let cursor_id = cursor.node().id();
        self.cursor_to_references
            .get(&cursor_id)
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle: *handle,
            })
    }

    fn get_file(&self, handle: FileHandle) -> Option<FileDescriptor> {
        self.files.get(&handle).cloned()
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
