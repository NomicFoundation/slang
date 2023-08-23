use super::{cst, parse_error::ParseError};

#[derive(Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) parse_tree: cst::Node,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn parse_tree(&self) -> cst::Node {
        return self.parse_tree.clone();
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        return &self.errors;
    }

    pub fn is_valid(&self) -> bool {
        return self.errors.is_empty();
    }
}
