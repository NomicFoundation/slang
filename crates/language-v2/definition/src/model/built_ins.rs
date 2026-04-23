use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Code, Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInContext {
    pub name: Identifier,
    pub scopes: Vec<BuiltInScope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInScope {
    pub name: Identifier,
    pub definitions: Vec<BuiltInDefinition>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInDefinition {
    pub name: Identifier,

    /// The version range this built-in is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// A verbatim Rust type to use in the definition of the variant in the
    /// internal enum if required for correct resolution or typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_parameter: Option<Code>,
}
