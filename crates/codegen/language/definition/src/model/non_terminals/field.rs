use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct FieldsErrorRecovery {
    pub terminator: Option<Identifier>,
    pub delimiters: Option<FieldDelimiters>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct FieldDelimiters {
    pub open: Identifier,
    pub close: Identifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum Field {
    Required {
        reference: Identifier,
    },
    Optional {
        reference: Identifier,

        enabled: Option<VersionSpecifier>,
    },
}
