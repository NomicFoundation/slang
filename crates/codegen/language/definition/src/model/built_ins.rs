use std::rc::Rc;

use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::model::VersionSpecifier;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, EnumDiscriminants, ParseInputTokens, WriteOutputTokens)]
pub enum BuiltIn {
    BuiltInFunction { item: Rc<BuiltInFunction> },
    BuiltInType { item: Rc<BuiltInType> },
    BuiltInVariable { item: Rc<BuiltInField> },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInType {
    pub name: String,
    pub fields: Vec<BuiltInField>,
    pub functions: Vec<BuiltInFunction>,
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInField {
    pub definition: String,
    pub enabled: Option<VersionSpecifier>,
}
