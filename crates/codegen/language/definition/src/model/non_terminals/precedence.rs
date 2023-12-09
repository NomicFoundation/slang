use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct PrecedenceItem {
    pub name: Identifier,

    pub enabled: Option<VersionSpecifier>,

    pub precedence_expressions: Vec<PrecedenceExpression>,
    pub primary_expressions: Vec<PrimaryExpression>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct PrecedenceExpression {
    pub name: Identifier,

    pub operators: Vec<PrecedenceOperator>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct PrecedenceOperator {
    pub model: OperatorModel,

    pub enabled: Option<VersionSpecifier>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Identifier, Field>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum OperatorModel {
    Prefix,
    Postfix,
    BinaryLeftAssociative,
    BinaryRightAssociative,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct PrimaryExpression {
    pub reference: Identifier,

    pub enabled: Option<VersionSpecifier>,
}
