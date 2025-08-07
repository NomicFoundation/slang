use std::collections::BTreeMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;
use metaslang_cst::text_index::TextIndex;

use crate::cst::{Cursor, NonterminalNode};
use crate::parser::{ParseError, ParseOutput};

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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tree(&self) -> &Rc<NonterminalNode> {
        &self.tree
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

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
