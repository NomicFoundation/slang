use std::collections::BTreeMap;
use std::rc::Rc;

use metaslang_cst::text_index::TextIndex;

use crate::cst::{Cursor, NonterminalNode};

#[derive(Clone)]
pub struct File {
    id: String,
    tree: Rc<NonterminalNode>,

    resolved_imports: BTreeMap<usize, String>,
}

impl File {
    pub(super) fn new(id: String, tree: Rc<NonterminalNode>) -> Self {
        Self {
            id,
            tree,

            resolved_imports: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tree(&self) -> &Rc<NonterminalNode> {
        &self.tree
    }

    pub fn create_tree_cursor(&self) -> Cursor {
        Rc::clone(&self.tree).cursor_with_offset(TextIndex::ZERO)
    }

    pub(super) fn resolve_import(&mut self, import_path: &Cursor, destination_file_id: String) {
        self.resolved_imports
            .insert(import_path.node().id(), destination_file_id);
    }

    pub(super) fn resolved_import(&self, import_path: &Cursor) -> Option<&String> {
        self.resolved_imports.get(&import_path.node().id())
    }
}
