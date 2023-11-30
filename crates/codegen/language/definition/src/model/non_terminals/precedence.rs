use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct PrecedenceItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub precedence_expressions: Vec<PrecedenceExpression>,
    pub primary_expressions: Vec<PrimaryExpression>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct PrecedenceExpression {
    pub name: Identifier,

    // TODO(#638): Remove this, once we adapt the DSL v1 codegen model to the new v2 definition.
    pub rule_name: Identifier,

    pub operators: Vec<PrecedenceOperator>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct PrecedenceOperator {
    pub model: OperatorModel,

    pub enabled: Option<VersionSpecifier>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Identifier, Field>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub enum OperatorModel {
    Prefix,
    Postfix,
    BinaryLeftAssociative,
    BinaryRightAssociative,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct PrimaryExpression {
    pub expression: Identifier,

    pub enabled: Option<VersionSpecifier>,
}
