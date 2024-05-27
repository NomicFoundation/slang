use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct StructItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Identifier, Field>,
}
