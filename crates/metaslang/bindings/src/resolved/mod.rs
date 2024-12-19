use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Debug, Display};
use std::rc::Rc;

use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;

use crate::{
    BindingGraphBuilder, BindingLocation, CursorID, FileDescriptor, FileHandle, GraphHandle,
};

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
        binding_graph: BindingGraphBuilder<KT>,
        resolved_references: HashMap<GraphHandle, Vec<GraphHandle>>,
    ) -> Rc<Self> {
        let mut files = HashMap::new();
        for handle in binding_graph.stack_graph.iter_files() {
            files.insert(
                handle,
                FileDescriptor::from(binding_graph.stack_graph[handle].name()),
            );
        }
        let mut definitions = BTreeMap::new();
        let mut references = BTreeMap::new();
        for handle in binding_graph.stack_graph.iter_nodes() {
            let graph_node = &binding_graph.stack_graph[handle];
            let Some(file) = graph_node.file() else {
                continue;
            };
            if graph_node.is_definition() {
                let cursor = binding_graph
                    .cursors
                    .get(&handle)
                    .expect("Definition to have a valid cursor")
                    .clone();
                let definiens = binding_graph.definitions_info[&handle].definiens.clone();
                definitions.insert(
                    handle,
                    DefinitionInfo {
                        file,
                        cursor,
                        definiens,
                    },
                );
            } else if graph_node.is_reference() {
                let cursor = binding_graph
                    .cursors
                    .get(&handle)
                    .expect("Reference to have a valid cursor")
                    .clone();
                references.insert(handle, ReferenceInfo { file, cursor });
            }
        }

        Rc::new(Self {
            files,
            definitions,
            references,
            cursor_to_definitions: binding_graph.cursor_to_definitions,
            cursor_to_references: binding_graph.cursor_to_references,
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

#[derive(Clone)]
pub struct Definition<KT: KindTypes + 'static> {
    owner: Rc<BindingGraph<KT>>,
    handle: GraphHandle,
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

#[derive(Clone)]
pub struct Reference<KT: KindTypes + 'static> {
    owner: Rc<BindingGraph<KT>>,
    handle: GraphHandle,
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
        self.owner.resolved_references[&self.handle]
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
