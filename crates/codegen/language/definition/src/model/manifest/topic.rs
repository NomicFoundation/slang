use crate::model::{Identifier, Item, Spanned};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;
use std::{path::PathBuf, rc::Rc};

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct Topic {
    pub title: Spanned<String>,
    pub notes_file: Option<Spanned<PathBuf>>,
    pub lexical_context: Option<Spanned<Identifier>>,

    pub items: Vec<Rc<Item>>,
}
