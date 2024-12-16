// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;

use metaslang_cst::text_index::TextIndex;

use crate::cst::{Cursor, Node};

#[derive(Clone)]
pub struct File {
    id: String,
    tree: Node,

    resolved_imports: BTreeMap<usize, String>,
}

impl File {
    pub(super) fn new(id: String, tree: Node) -> Self {
        Self {
            id,
            tree,

            resolved_imports: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tree(&self) -> &Node {
        &self.tree
    }

    pub fn create_tree_cursor(&self) -> Cursor {
        self.tree.clone().cursor_with_offset(TextIndex::ZERO)
    }

    pub(super) fn resolve_import(&mut self, import_path: &Cursor, destination_file_id: String) {
        self.resolved_imports
            .insert(import_path.node().id(), destination_file_id);
    }

    pub(super) fn resolved_import(&self, import_path: &Cursor) -> Option<&String> {
        self.resolved_imports.get(&import_path.node().id())
    }
}
