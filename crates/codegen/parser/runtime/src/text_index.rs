use std::{
    fmt::Display,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
};

use serde::Serialize;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TextIndex {
    pub utf8: usize,
    pub utf16: usize,
    pub char: usize,
}

impl PartialOrd for TextIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.utf8.partial_cmp(&other.utf8)
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

impl From<&str> for TextIndex {
    fn from(s: &str) -> Self {
        let mut utf8 = 0;
        let mut utf16 = 0;
        let mut char = 0;
        for c in s.chars() {
            utf8 += c.len_utf8();
            utf16 += c.len_utf16();
            char += 1;
        }
        Self { utf8, utf16, char }
    }
}

impl From<&String> for TextIndex {
    fn from(s: &String) -> Self {
        let mut utf8 = 0;
        let mut utf16 = 0;
        let mut char = 0;
        for c in s.chars() {
            utf8 += c.len_utf8();
            utf16 += c.len_utf16();
            char += 1;
        }
        Self { utf8, utf16, char }
    }
}

impl Add for TextIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            utf8: self.utf8 + rhs.utf8,
            utf16: self.utf16 + rhs.utf16,
            char: self.char + rhs.char,
        }
    }
}

impl Sub for TextIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            utf8: self.utf8 - rhs.utf8,
            utf16: self.utf16 - rhs.utf16,
            char: self.char - rhs.char,
        }
    }
}

impl AddAssign for TextIndex {
    fn add_assign(&mut self, rhs: Self) {
        self.utf8 += rhs.utf8;
        self.utf16 += rhs.utf16;
        self.char += rhs.char;
    }
}

impl SubAssign for TextIndex {
    fn sub_assign(&mut self, rhs: Self) {
        self.utf8 -= rhs.utf8;
        self.utf16 -= rhs.utf16;
        self.char -= rhs.char;
    }
}

pub type TextRange = Range<TextIndex>;

pub trait TextRangeExtensions {
    fn utf8(&self) -> Range<usize>;
    fn utf16(&self) -> Range<usize>;
    fn char(&self) -> Range<usize>;
}

impl TextRangeExtensions for TextRange {
    fn utf8(&self) -> Range<usize> {
        self.start.utf8..self.end.utf8
    }

    fn utf16(&self) -> Range<usize> {
        self.start.utf16..self.end.utf16
    }

    fn char(&self) -> Range<usize> {
        self.start.char..self.end.char
    }
}
