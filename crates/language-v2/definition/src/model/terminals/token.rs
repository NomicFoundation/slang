use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, TokenScanner, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TokenItem {
    pub name: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    pub definitions: Vec<TokenDefinition>,
}

impl TokenItem {
    pub fn unique_symbol(&self) -> Option<String> {
        match self.definitions.as_slice() {
            [definition] => definition.scanner.unique_symbol(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TokenDefinition {
    pub scanner: TokenScanner,

    // TODO(v2): instead of the lexer failing when this suffix is found,
    // it should report more specific diagnostics, and create the lexeme successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_suffix: Option<TokenScanner>,
}
