use language_v2_definition::model::{Language, VersionSpecifier};
use serde::Serialize;

#[derive(Serialize)]
pub struct BuiltInContextModel {
    pub name: String,
    pub scopes: Vec<BuiltInScopeModel>,
}

#[derive(Serialize)]
pub struct BuiltInScopeModel {
    pub name: String,
    pub definitions: Vec<BuiltInDefinitionModel>,
}

#[derive(Serialize)]
pub struct BuiltInDefinitionModel {
    pub name: String,
    pub enabled: VersionSpecifier,
    pub internal_parameter: Option<String>,
}

pub fn build_built_ins_model(language: &Language) -> Vec<BuiltInContextModel> {
    language
        .built_ins
        .iter()
        .map(|context| BuiltInContextModel {
            name: context.name.to_string(),
            scopes: context
                .scopes
                .iter()
                .map(|scope| BuiltInScopeModel {
                    name: scope.name.to_string(),
                    definitions: scope
                        .definitions
                        .iter()
                        .map(|def| BuiltInDefinitionModel {
                            name: def.name.to_string(),
                            enabled: def.enabled.clone().unwrap_or(VersionSpecifier::Always),
                            internal_parameter: def.internal_parameter.clone(),
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect()
}
