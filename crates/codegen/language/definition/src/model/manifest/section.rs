use crate::model::{Spanned, Topic};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct Section {
    pub title: Spanned<String>,
    pub topics: Vec<Topic>,
}
