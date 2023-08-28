use std::mem;
use std::ops::Range;

use super::super::text_index::TextIndex;

pub struct Stream<'s> {
    source: &'s str,
    position: TextIndex,
    undo_position: Option<TextIndex>,
}

impl<'s> Stream<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: Default::default(),
            undo_position: None,
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
