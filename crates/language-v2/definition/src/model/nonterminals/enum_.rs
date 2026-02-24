use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, ParserOptions, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct EnumItem {
    pub name: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    pub variants: Vec<EnumVariant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser_options: Option<ParserOptions>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct EnumVariant {
    pub reference: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}
