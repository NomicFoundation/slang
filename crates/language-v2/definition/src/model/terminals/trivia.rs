use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, TokenScanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Identifier,

    /// What the trivia is, whitespace if unset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TriviaCategory>,

    pub scanner: TokenScanner,
}

impl TriviaItem {
    /// What the trivia is, defaulting to whitespace.
    pub fn category(&self) -> TriviaCategory {
        self.category.unwrap_or_default()
    }
}

/// What a trivia item is.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub enum TriviaCategory {
    /// Whitespace, including line breaks.
    #[default]
    Whitespace,
    /// A regular comment, like `// ...` or `/* ... */`.
    RegularComment,
    /// A `NatSpec` comment, documenting the code that follows it, like `/// ...` or `/** ... */`.
    NatSpecComment,
}
