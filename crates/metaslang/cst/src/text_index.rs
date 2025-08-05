//! Represents a position within a text document, tracking multiple coordinate systems.
//!
//! `TextIndex` keeps track of:
//! - The number of UTF-8 bytes (`utf8`)
//! - The number of UTF-16 code units (`utf16`)
//! - The line number (`line`)
//! - The column number (`column`)
//!
//! This structure is useful for mapping between different text representations and for
//! reporting precise locations in source code (e.g., for diagnostics, highlighting, etc.).
//!
//! # Examples
//! ```
//! use crate::TextIndex;
//!
//! let mut idx = TextIndex::ZERO;
//! idx.advance_str("hello\nworld");
//! assert_eq!(idx.line, 1);
//! assert_eq!(idx.column, 5);
//! ```
//!
//! # Notes
//! - The `advance` and `advance_str` methods update the index as you process characters or strings.
//! - Line breaks are recognized as LF (`\n`), CR (`\r`), Line Separator (`\u{2028}`), or Paragraph Separator (`\u{2029}`).
//! - Handles CRLF (`\r\n`) as a single line break.

use std::fmt::Display;
use std::ops::{Add, AddAssign, Range};

use serde::Serialize;

/// Represents a position within a text document, tracking multiple coordinate systems.
#[derive(Default, Hash, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TextIndex {
    /// Offset in UTF-8 bytes from the start of the text.
    pub utf8: usize,
    /// Offset in UTF-16 code units from the start of the text.
    pub utf16: usize,
    /// Zero-based line number.
    pub line: usize,
    /// Zero-based column number within the line.
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
                // Ignore for now, we will increment the line number when we process the \n
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

    /// Advances the index by processing a string character by character.
    pub fn advance_str(&mut self, text: &str) {
        let mut iter = text.chars().peekable();
        while let Some(c) = iter.next() {
            let n = iter.peek();
            self.advance(c, n);
        }
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
        result.advance_str(s.as_ref());
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

/// Represents a range of text indices, useful for defining spans in text documents.
pub type TextRange = Range<TextIndex>;

/// Extensions to obtain character and line ranges from a [`TextRange`].
pub trait TextRangeExtensions {
    /// Returns a range of utf8 characters.
    fn utf8(&self) -> Range<usize>;
    /// Returns a range of utf16 characters.
    fn utf16(&self) -> Range<usize>;
    /// Returns a range of lines.
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
