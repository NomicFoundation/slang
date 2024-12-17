use crate::cst::{Cursor, Node, TextIndex};
use crate::parser::ParseError;

#[derive(Clone, Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) tree: Node,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn tree(&self) -> &Node {
        &self.tree
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    pub fn create_tree_cursor(&self) -> Cursor {
        self.tree.clone().cursor_with_offset(TextIndex::ZERO)
    }
}
