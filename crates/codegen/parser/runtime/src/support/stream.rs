use std::ops::Range;

use super::super::text_index::TextIndex;

pub struct Stream<'s> {
    source: &'s str,
    position: TextIndex,
    undo_position: TextIndex,
    has_undo: bool,
}

impl<'s> Stream<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: Default::default(),
            undo_position: Default::default(),
            has_undo: false,
        }
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
        self.has_undo = true;
        self.undo_position = self.position;

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
        if !self.has_undo {
            panic!("No undo position");
        }
        self.position = self.undo_position;
        self.has_undo = false;
    }

    pub fn content(&self, range: Range<usize>) -> String {
        self.source[range].to_owned()
    }
}
