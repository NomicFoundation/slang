use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct ParserOptions {
    pub inline: bool,
    pub pubb: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<String>,
}
