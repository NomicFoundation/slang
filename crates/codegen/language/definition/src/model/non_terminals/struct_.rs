use crate::model::{Field, FieldsErrorRecovery, Identifier, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct StructItem {
    pub name: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Spanned<Identifier>, Field>,
}
