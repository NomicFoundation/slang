// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::{collections::BTreeSet, ops::Range, rc::Rc};

use super::{
    cst,
    language::{render_error_report, TextRange},
};

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
    pub(crate) range: TextRange,
    pub(crate) expected: BTreeSet<String>,
}

impl ParseError {
    pub fn range(&self) -> &TextRange {
        return &self.range;
    }

    pub fn utf8_range(&self) -> Range<usize> {
        self.range.start.utf8..self.range.end.utf8
    }

    pub fn utf16_range(&self) -> Range<usize> {
        self.range.start.utf16..self.range.end.utf16
    }

    pub fn char_range(&self) -> Range<usize> {
        self.range.start.char..self.range.end.char
    }

    pub fn expected(&self) -> &BTreeSet<String> {
        return &self.expected;
    }

    pub fn to_error_report(&self, source_id: &str, source: &str, with_colour: bool) -> String {
        return render_error_report(self, source_id, source, with_colour);
    }
}
