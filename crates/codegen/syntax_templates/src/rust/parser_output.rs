use std::{collections::BTreeSet, rc::Rc};

use super::{cst, language::render_error_report};

#[derive(PartialEq)]
pub struct ParseOutput {
    pub(crate) parse_tree: Option<Rc<cst::Node>>,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn parse_tree(&self) -> Option<Rc<cst::Node>> {
        return self.parse_tree.clone();
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        return &self.errors;
    }

    pub fn is_valid(&self) -> bool {
        return self.parse_tree.is_some() && self.errors.is_empty();
    }
}

#[derive(PartialEq)]
pub struct ParseError {
    pub(crate) position: usize,
    pub(crate) expected: BTreeSet<String>,
}

impl ParseError {
    pub fn position(&self) -> usize {
        return self.position;
    }

    pub fn expected(&self) -> &BTreeSet<String> {
        return &self.expected;
    }

    pub fn to_error_report(&self, source_id: &str, source: &str, with_colour: bool) -> String {
        return render_error_report(self, source_id, source, with_colour);
    }
}
