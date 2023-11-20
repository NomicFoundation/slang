// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::{cst, cursor::Cursor, parse_error::ParseError, text_index::TextIndex};

#[derive(Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) parse_tree: cst::Node,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn tree(&self) -> cst::Node {
        return self.parse_tree.clone();
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        return &self.errors;
    }

    pub fn is_valid(&self) -> bool {
        return self.errors.is_empty();
    }

    /// Creates a cursor that starts at the root of the parse tree.
    pub fn create_tree_cursor(&self) -> Cursor {
        return self.parse_tree.cursor_with_offset(TextIndex::ZERO);
    }
}
