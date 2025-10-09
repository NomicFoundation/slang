use std::mem;
use std::ops::Range;

use super::ParserResult;
use crate::cst::{TerminalKind, TextIndex};
use crate::parser::ParseError;

#[derive(Debug)]
struct CachedParserResult {
    start_position: usize,
    end_position: usize,
    result: ParserResult,
}

#[derive(Debug)]
pub struct ParserContext<'s> {
    source: &'s str,
    position: usize,
    undo_position: Option<usize>,
    errors: Vec<ParseError>,
    closing_delimiters: Vec<TerminalKind>,
    last_text_index: TextIndex,
    leading_trivia_cache: Option<CachedParserResult>,
}

#[derive(Copy, Clone)]
pub struct Marker {
    position: usize,
    err_len: usize,
}

impl<'s> ParserContext<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: 0usize,
            undo_position: None,
            errors: vec![],
            closing_delimiters: vec![],
            last_text_index: TextIndex::ZERO,
            leading_trivia_cache: None,
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

    /// Creates a RAII guard that will pop the closing delimiter when dropped.
    pub(crate) fn open_delim<'a>(
        &'a mut self,
        closing_delim: TerminalKind,
    ) -> DelimiterGuard<'a, 's> {
        self.closing_delimiters.push(closing_delim);

        DelimiterGuard {
            input: self,
            closing_delim,
        }
    }

    pub fn closing_delimiters(&self) -> &[TerminalKind] {
        &self.closing_delimiters
    }

    pub fn text_index_at(&mut self, position: usize) -> TextIndex {
        // This is a minor optimization: we remember the last computed TextIndex
        // and if the requested position is after, we start from that last
        // index and avoid advancing over the same characters again. Otherwise,
        // we do start from the beginning.
        let mut text_index = if self.last_text_index.utf8 <= position {
            self.last_text_index
        } else {
            TextIndex::ZERO
        };
        let mut from_iter = self.source[text_index.utf8..].chars();
        let Some(mut c) = from_iter.next() else {
            return text_index;
        };
        let mut next_c = from_iter.next();
        loop {
            if text_index.utf8 >= position {
                break;
            }
            text_index.advance(c, next_c.as_ref());
            c = match next_c {
                Some(ch) => ch,
                None => break,
            };
            next_c = from_iter.next();
        }
        self.last_text_index = text_index;
        text_index
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<char> {
        self.undo_position = Some(self.position);

        if let Some(c) = self.peek() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn undo(&mut self) {
        let position = mem::take(&mut self.undo_position).expect("No undo position");
        self.position = position;
    }

    pub fn content(&self, range: Range<usize>) -> &str {
        &self.source[range]
    }

    pub fn cached_leading_trivia_or(
        &mut self,
        f: impl FnOnce(&mut Self) -> ParserResult,
    ) -> ParserResult {
        let position = self.position();
        if let Some(cache) = &self.leading_trivia_cache {
            if cache.start_position == position {
                let result = cache.result.clone();
                self.set_position(cache.end_position);
                return result;
            }
        }

        let result = f(self);
        self.leading_trivia_cache = Some(CachedParserResult {
            start_position: position,
            end_position: self.position(),
            result: result.clone(),
        });
        result
    }
}

pub(crate) struct DelimiterGuard<'a, 's> {
    pub(crate) input: &'a mut ParserContext<'s>,
    closing_delim: TerminalKind,
}

impl Drop for DelimiterGuard<'_, '_> {
    fn drop(&mut self) {
        let popped = self.input.closing_delimiters.pop();
        debug_assert_eq!(popped, Some(self.closing_delim));
    }
}

impl<'s> DelimiterGuard<'_, 's> {
    pub(crate) fn ctx(&mut self) -> &mut ParserContext<'s> {
        self.input
    }
}
