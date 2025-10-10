use std::rc::Rc;

use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use semver::Version;
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::model::VersionSpecifier;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInContext {
    pub name: String,
    pub definitions: Vec<BuiltIn>,
}

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

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInType {
    pub name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<BuiltInField>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<BuiltInFunction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInField {
    pub definition: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}

pub fn filter_built_ins_for_version(built_ins: &[BuiltIn], version: &Version) -> Vec<BuiltIn> {
    built_ins
        .iter()
        .filter_map(|built_in| match built_in {
            BuiltIn::BuiltInFunction { item } => filter_built_in_function(item, version)
                .map(|item| BuiltIn::BuiltInFunction { item: item.into() }),
            BuiltIn::BuiltInType { item } => filter_built_in_type(item, version)
                .map(|item| BuiltIn::BuiltInType { item: item.into() }),
            BuiltIn::BuiltInVariable { item } => filter_built_in_field(item, version)
                .map(|item| BuiltIn::BuiltInVariable { item: item.into() }),
        })
        .collect()
}

fn filter_built_in_function(
    function: &BuiltInFunction,
    version: &Version,
) -> Option<BuiltInFunction> {
    if function
        .enabled
        .as_ref()
        .is_none_or(|enabled| enabled.contains(version))
    {
        Some(function.clone())
    } else {
        None
    }
}

fn filter_built_in_field(field: &BuiltInField, version: &Version) -> Option<BuiltInField> {
    if field
        .enabled
        .as_ref()
        .is_none_or(|enabled| enabled.contains(version))
    {
        Some(field.clone())
    } else {
        None
    }
}

fn filter_built_in_type(typ: &BuiltInType, version: &Version) -> Option<BuiltInType> {
    if typ
        .enabled
        .as_ref()
        .is_none_or(|enabled| enabled.contains(version))
    {
        let fields = typ
            .fields
            .iter()
            .filter_map(|field| filter_built_in_field(field, version))
            .collect();
        let functions = typ
            .functions
            .iter()
            .filter_map(|function| filter_built_in_function(function, version))
            .collect();

        Some(BuiltInType {
            name: typ.name.clone(),
            fields,
            functions,
            enabled: typ.enabled.clone(),
        })
    } else {
        None
    }
}
