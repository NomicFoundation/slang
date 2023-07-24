use std::collections::BTreeSet;

use codegen_schema::types::{LanguageDefinitionRef, ProductionDefinition};
use serde::Serialize;

use crate::templates::TemplateContext;

#[derive(Serialize)]
pub struct NodeKindsTemplate {
    pub enum_name: String,
    pub enum_variants: Vec<String>,
}

impl TemplateContext for NodeKindsTemplate {
    const TEMPLATE_RELATIVE_PATH: &'static str = "nodes/kinds/template.tera";
}

impl NodeKindsTemplate {
    pub fn collect_production_kinds(language: &LanguageDefinitionRef) -> Self {
        let mut enum_variants = vec![];

        for production in language.productions.values() {
            if production.inlined {
                continue;
            }

            enum_variants.push(production.name.to_owned());
        }

        enum_variants.sort();

        return Self {
            enum_name: "ProductionKind".to_string(),
            enum_variants,
        };
    }

    pub fn collect_rule_kinds(language: &LanguageDefinitionRef) -> Self {
        // Use a BTreeSet to deduplicate operator names:
        let mut enum_variants = BTreeSet::new();

        for production in language.productions.values() {
            if production.inlined {
                continue;
            }

            match &production.definition {
                ProductionDefinition::Parser { .. } | ProductionDefinition::TriviaParser { .. } => {
                    enum_variants.insert(production.name.to_owned());
                }
                ProductionDefinition::PrecedenceParser { version_map, .. } => {
                    enum_variants.insert(production.name.to_owned());

                    match version_map {
                        codegen_schema::types::VersionMap::Unversioned(unversioned) => {
                            for expression in &unversioned.operator_expressions {
                                enum_variants.insert(expression.name.to_owned());
                            }
                        }
                        codegen_schema::types::VersionMap::Versioned(versioned) => {
                            for version in versioned.values() {
                                if let Some(version) = version {
                                    for expression in &version.operator_expressions {
                                        enum_variants.insert(expression.name.to_owned());
                                    }
                                }
                            }
                        }
                    };
                }
                ProductionDefinition::Scanner { .. } => {}
            };
        }

        // BTreeSet iterators are always ordered, so we don't need to sort.

        return Self {
            enum_name: "RuleKind".to_string(),
            enum_variants: enum_variants.into_iter().collect(),
        };
    }

    pub fn collect_token_kinds(language: &LanguageDefinitionRef) -> Self {
        let mut enum_variants = vec![];

        for production in language.productions.values() {
            if production.inlined {
                continue;
            }

            match production.definition {
                ProductionDefinition::Scanner { .. } => {
                    enum_variants.push(production.name.to_owned());
                }
                ProductionDefinition::Parser { .. }
                | ProductionDefinition::PrecedenceParser { .. }
                | ProductionDefinition::TriviaParser { .. } => {}
            };
        }

        enum_variants.sort();

        // Insert "SKIPPED" as the first token kind (after sorting):
        enum_variants.insert(0, "SKIPPED".to_owned());

        return Self {
            enum_name: "TokenKind".to_string(),
            enum_variants,
        };
    }
}
