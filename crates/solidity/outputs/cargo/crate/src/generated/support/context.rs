// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::mem;
use std::ops::Range;

use crate::kinds::TokenKind;
use crate::parse_error::ParseError;

use super::super::text_index::TextIndex;

pub struct ParserContext<'s> {
    source: &'s str,
    position: TextIndex,
    undo_position: Option<TextIndex>,
    errors: Vec<ParseError>,
    closing_delimiters: Vec<TokenKind>,
}

#[derive(Copy, Clone)]
pub struct Marker {
    position: TextIndex,
    err_len: usize,
}

impl Marker {
    pub fn err_len(&self) -> usize {
        self.err_len
    }
}

impl<'s> ParserContext<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: Default::default(),
            undo_position: None,
            errors: vec![],
            closing_delimiters: vec![],
        }
    }

    pub fn mark(&self) -> Marker {
        Marker {
            position: self.position,
            err_len: self.errors.len(),
        }
    }

    pub fn rewind(&mut self, marker: Marker) {
        assert!(marker.position <= self.position);

        self.position = marker.position;
        self.errors.truncate(marker.err_len);
    }

    pub fn emit(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn errors_since(&self, marker: Marker) -> &[ParseError] {
        &self.errors[marker.err_len..]
    }
    pub fn extend_errors(&mut self, errors: Vec<ParseError>) {
        self.errors.extend(errors);
    }

    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }

    pub fn expect_closing(&mut self, closing_delim: TokenKind) {
        self.closing_delimiters.push(closing_delim);
    }

    pub fn pop_closing(&mut self, closing_delim: TokenKind) {
        let popped = self.closing_delimiters.pop();
        assert_eq!(popped, Some(closing_delim));
    }

    pub fn closing_delimiters(&self) -> &[TokenKind] {
        &self.closing_delimiters
    }

    pub fn position(&self) -> TextIndex {
        self.position
    }

    pub fn set_position(&mut self, position: TextIndex) {
        self.position = position;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position.utf8..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        self.undo_position = Some(self.position);

        if let Some(c) = self.peek() {
            self.position.utf8 += c.len_utf8();
            self.position.utf16 += c.len_utf16();
            self.position.char += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn undo(&mut self) {
        let position = mem::take(&mut self.undo_position).expect("No undo position");
        self.position = position;
    }

    pub fn content(&self, range: Range<usize>) -> String {
        self.source[range].to_owned()
    }
}
