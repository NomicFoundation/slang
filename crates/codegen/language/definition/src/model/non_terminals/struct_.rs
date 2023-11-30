use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct StructItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Identifier, Field>,
}
