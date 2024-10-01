// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst::{Cursor, Node, TextIndex};
use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) parse_tree: Node,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn tree(&self) -> Node {
        self.parse_tree.clone()
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    pub fn create_tree_cursor(&self) -> Cursor {
        self.parse_tree.cursor_with_offset(TextIndex::ZERO)
    }
}
