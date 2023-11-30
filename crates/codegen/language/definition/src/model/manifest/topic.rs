use crate::model::{Identifier, Item};

use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct Topic {
    pub title: String,
    pub notes_file: Option<String>,
    pub lexical_context: Option<Identifier>,

    pub items: Vec<Rc<Item>>,
}
