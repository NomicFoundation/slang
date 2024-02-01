use napi_derive::napi;

use crate::napi_interface::{RustTextIndex, RustTextRange};

#[napi(object, namespace = "text_index")]
#[derive(Copy, Clone)]
pub struct TextIndex {
    pub utf8: u32,
    pub utf16: u32,
    pub char: u32,
}

impl From<&RustTextIndex> for TextIndex {
    fn from(value: &RustTextIndex) -> Self {
        // We only support 32-byte indices on TS side.
        #[allow(clippy::cast_possible_truncation)]
        Self {
            utf8: value.utf8 as u32,
            utf16: value.utf16 as u32,
            char: value.char as u32,
        }
    }
}

impl From<&TextIndex> for RustTextIndex {
    fn from(value: &TextIndex) -> Self {
        Self {
            utf8: value.utf8 as usize,
            utf16: value.utf16 as usize,
            char: value.char as usize,
        }
    }
}

#[napi(object, namespace = "text_index")]
#[derive(Copy, Clone)]
pub struct TextRange {
    pub start: TextIndex,
    pub end: TextIndex,
}

impl From<&RustTextRange> for TextRange {
    fn from(value: &RustTextRange) -> Self {
        Self {
            start: (&value.start).into(),
            end: (&value.end).into(),
        }
    }
}
