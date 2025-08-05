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
#[cfg(feature = "__private_testing_utils")]
use metaslang_cst::nodes::NodeId;
pub use reference::Reference;
use resolver::Resolver;

use crate::builder::{ExtendedStackGraph, FileDescriptor, GraphHandle};

/// Represents the binding graph for Slang.
pub struct BindingGraph<KT: KindTypes + 'static> {
    graph: ExtendedStackGraph<KT>,
    resolver: Rc<RefCell<Resolver>>,
}

impl<KT: KindTypes + 'static> BindingGraph<KT> {
    pub(crate) fn build(graph: ExtendedStackGraph<KT>) -> Rc<Self> {
        let resolver = Resolver::new(&graph);

        Rc::new(Self {
            graph,
            resolver: Rc::new(RefCell::new(resolver)),
        })
    }

    /// Tries to resolve the identifier terminal pointed at by the provided cursor to a definition.
    /// If successful, returns the definition. Otherwise, returns `None`.
    ///
    /// For more information on identifier terminals, see [`TerminalKindExtensions::is_identifier`](`metaslang_cst::kinds::TerminalKindExtensions::is_identifier`).
    pub fn definition_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Definition<KT>> {
        self.graph
            .definition_by_node_id(cursor.node().id())
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle,
            })
    }

    /// Tries to resolve the identifier terminal pointed at by the provided cursor to a reference.
    /// If successful, returns the reference. Otherwise, returns `None`.
    ///
    /// For more information on identifier terminals, see [`TerminalKindExtensions::is_identifier`](`metaslang_cst::kinds::TerminalKindExtensions::is_identifier`).
    pub fn reference_at(self: &Rc<Self>, cursor: &Cursor<KT>) -> Option<Reference<KT>> {
        self.graph
            .reference_by_node_id(cursor.node().id())
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle,
            })
    }

    fn resolve_reference(self: &Rc<Self>, handle: GraphHandle) -> Vec<Definition<KT>> {
        let mut resolver = self.resolver.borrow_mut();
        let definitions = resolver.resolve_single(&self.graph, handle);
        definitions
            .iter()
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle: *handle,
            })
            .collect()
    }

    fn resolve_definition(self: &Rc<Self>, handle: GraphHandle) -> Vec<Reference<KT>> {
        let mut resolver = self.resolver.borrow_mut();
        resolver.ensure_all_references_resolved(&self.graph);
        let references = resolver.definition_to_references(handle);
        references
            .iter()
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle: *handle,
            })
            .collect()
    }
}

#[cfg(feature = "__private_testing_utils")]
#[allow(missing_docs)]
impl<KT: KindTypes + 'static> BindingGraph<KT> {
    pub fn all_definitions(self: &Rc<Self>) -> impl Iterator<Item = Definition<KT>> + '_ {
        self.graph.iter_definitions().map(|handle| Definition {
            owner: Rc::clone(self),
            handle,
        })
    }

    pub fn all_references(self: &Rc<Self>) -> impl Iterator<Item = Reference<KT>> + '_ {
        self.graph.iter_references().map(|handle| Reference {
            owner: Rc::clone(self),
            handle,
        })
    }

    pub fn definition_by_node_id(self: &Rc<Self>, node_id: NodeId) -> Option<Definition<KT>> {
        self.graph
            .definition_by_node_id(node_id)
            .map(|handle| Definition {
                owner: Rc::clone(self),
                handle,
            })
    }

    pub fn reference_by_node_id(self: &Rc<Self>, node_id: NodeId) -> Option<Reference<KT>> {
        self.graph
            .reference_by_node_id(node_id)
            .map(|handle| Reference {
                owner: Rc::clone(self),
                handle,
            })
    }
}

struct DisplayCursor<'a, KT: KindTypes + 'static> {
    cursor: &'a Cursor<KT>,
    file: FileDescriptor,
}

impl<KT: KindTypes + 'static> fmt::Display for DisplayCursor<'_, KT> {
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
