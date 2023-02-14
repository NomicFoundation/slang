// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use super::{
    cst,
    language::{render_error_report, ParseError},
};

#[derive(PartialEq)]
pub struct ParserOutput {
    pub(crate) parse_tree: Option<Rc<cst::Node>>,
    pub(crate) errors: Vec<ParseError>,
}

impl ParserOutput {
    pub fn parse_tree(&self) -> Option<Rc<cst::Node>> {
        self.parse_tree.clone()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn errors_as_strings(
        &self,
        source_id: &str,
        source: &str,
        with_colour: bool,
    ) -> Vec<String> {
        return self
            .errors
            .iter()
            .map(|error| render_error_report(error, source_id, source, with_colour))
            .collect();
    }

    pub fn is_valid(&self) -> bool {
        self.parse_tree.is_some() && self.errors.is_empty()
    }
}
