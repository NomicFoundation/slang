use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

/// Options that are used during parser generation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct ParserOptions {
    /// Whether to make the parsing rule inlined, this can help
    /// resolve conflicts.
    pub inline: bool,

    /// Whether to make the parsing rule public, generating a parser
    /// that can be used from outside the module.    
    pub public: bool,

    /// Verbatim code to insert into the parser rule.
    ///
    /// Helpful to solve complex parsing situations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<String>,
}
