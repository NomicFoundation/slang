use std::fmt::Display;
use std::ops::{Add, AddAssign, Range};

use serde::Serialize;

#[derive(Default, Hash, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TextIndex {
    pub utf8: usize,
    pub utf16: usize,
    pub line: usize,
    pub column: usize,
}

impl TextIndex {
    /// Shorthand for `TextIndex { utf8: 0, utf16: 0, line: 0, char: 0 }`.
    pub const ZERO: TextIndex = TextIndex {
        utf8: 0,
        utf16: 0,
        line: 0,
        column: 0,
    };

    /// Advances the index, accounting for lf/nl/ls/ps characters and combinations.
    /// This is *not* derived from the definition of 'newline' in the language definition,
    /// nor is it a complete implementation of the Unicode line breaking algorithm.
    #[inline]
    pub fn advance(&mut self, c: char, next: Option<&char>) {
        self.utf8 += c.len_utf8();
        self.utf16 += c.len_utf16();
        match (c, next) {
            ('\r', Some('\n')) => {
                // Ignore for now, we will increment the line number whe we process the \n
            }
            ('\n' | '\r' | '\u{2028}' | '\u{2029}', _) => {
                self.line += 1;
                self.column = 0;
            }
            _ => {
                self.column += 1;
            }
        }
    }

    pub fn advance_str(&mut self, text: &str) {
        text.chars().for_each(|c| {
            if c != '\r' {
                self.advance(c, None);
            }
        });
    }
}

impl PartialOrd for TextIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TextIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.utf8.cmp(&other.utf8)
    }
}

impl Display for TextIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.utf8.fmt(f)
    }
}

impl<T: AsRef<str>> From<T> for TextIndex {
    fn from(s: T) -> Self {
        let mut result = Self::ZERO;
        let mut iter = s.as_ref().chars().peekable();
        while let Some(c) = iter.next() {
            let n = iter.peek();
            result.advance(c, n);
        }
        result
    }
}

impl Add for TextIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl AddAssign for TextIndex {
    fn add_assign(&mut self, rhs: Self) {
        self.utf8 += rhs.utf8;
        self.utf16 += rhs.utf16;
        if rhs.line > 0 {
            self.line += rhs.line;
            self.column = rhs.column;
        } else {
            self.column += rhs.column;
        }
    }
}

impl std::iter::Sum for TextIndex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

pub type TextRange = Range<TextIndex>;

pub trait TextRangeExtensions {
    fn utf8(&self) -> Range<usize>;
    fn utf16(&self) -> Range<usize>;
    fn line(&self) -> Range<usize>;
}

impl TextRangeExtensions for TextRange {
    fn utf8(&self) -> Range<usize> {
        self.start.utf8..self.end.utf8
    }

    fn utf16(&self) -> Range<usize> {
        self.start.utf16..self.end.utf16
    }

    fn line(&self) -> Range<usize> {
        self.start.line..self.end.line
    }
}
