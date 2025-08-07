use std::collections::BTreeMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;
use metaslang_cst::text_index::TextIndex;

use crate::cst::{Cursor, NonterminalNode};
use crate::parser::{ParseError, ParseOutput};

/// A single source file in the compilation unit.
#[derive(Clone)]
pub struct File {
    id: String,
    tree: Rc<NonterminalNode>,
    errors: Vec<ParseError>,

    resolved_imports: BTreeMap<NodeId, String>,
}

impl File {
    pub(super) fn create(id: String, parse_output: ParseOutput) -> Self {
        let ParseOutput { tree, errors } = parse_output;

        Self {
            id,
            tree,
            errors,

            resolved_imports: BTreeMap::new(),
        }
    }

    /// Returns the unique identifier of this file.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the syntax tree of this file.
    pub fn tree(&self) -> &Rc<NonterminalNode> {
        &self.tree
    }

    /// Returns a list of all errors encountered during parsing this file.
    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    /// Creates a cursor for traversing the syntax tree of this file.
    pub fn create_tree_cursor(&self) -> Cursor {
        Rc::clone(&self.tree).create_cursor(TextIndex::ZERO)
    }

    pub(super) fn resolve_import(&mut self, import_path: &Cursor, destination_file_id: String) {
        self.resolved_imports
            .insert(import_path.node().id(), destination_file_id);
    }

    pub(super) fn resolved_import(&self, import_path: &Cursor) -> Option<&String> {
        self.resolved_imports.get(&import_path.node().id())
    }

    #[allow(dead_code)]
    pub(crate) fn resolved_import_by_node_id(
        &self,
        import_path_node_id: NodeId,
    ) -> Option<&String> {
        self.resolved_imports.get(&import_path_node_id)
    }
}
