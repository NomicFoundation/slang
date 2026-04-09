use language_v2_definition::model::Language;
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
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect()
}
