use std::collections::HashMap;

use codegen_ebnf::EbnfSerializer;
use codegen_schema::types::{LanguageDefinitionRef, ProductionDefinition, ProductionRef};
use itertools::Itertools;
use serde::Serialize;

use crate::templates::TemplateContext;

#[derive(Serialize)]
pub struct NodeKindsTemplate {
    pub type_name: String,
    pub variants: Vec<NodeKindsVariant>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodeKindsVariant {
    pub name: String,
    pub documentation: String,
}

impl TemplateContext for NodeKindsTemplate {
    const TEMPLATE_RELATIVE_PATH: &'static str = "nodes/kinds/template.tera";
}

impl NodeKindsTemplate {
    pub fn collect_production_kinds(language: &LanguageDefinitionRef) -> Self {
        let mut variants = vec![];

        for production in language.productions.values() {
            collect_top_level(language, production, &mut variants);
        }

        variants.sort();

        return Self {
            type_name: "ProductionKind".to_string(),
            variants,
        };
    }

    pub fn collect_rule_kinds(language: &LanguageDefinitionRef) -> Self {
        let mut variants = vec![];

        for production in language.productions.values() {
            match &production.definition {
                ProductionDefinition::Parser { .. }
                | ProductionDefinition::PrecedenceParser { .. }
                | ProductionDefinition::TriviaParser { .. } => {
                    collect_recursive(language, production, &mut variants);
                }
                ProductionDefinition::Scanner { .. } => {
                    // Skip
                }
            };
        }

        variants.sort();

        return Self {
            type_name: "RuleKind".to_string(),
            variants,
        };
    }

    pub fn collect_token_kinds(language: &LanguageDefinitionRef) -> Self {
        let mut variants = vec![];

        for production in language.productions.values() {
            match production.definition {
                ProductionDefinition::Scanner { .. } => {
                    collect_top_level(language, production, &mut variants);
                }
                ProductionDefinition::Parser { .. }
                | ProductionDefinition::PrecedenceParser { .. }
                | ProductionDefinition::TriviaParser { .. } => {
                    // Skip
                }
            };
        }

        variants.sort();

        variants.insert(
            0, // Insert "SKIPPED" as the first token kind (after sorting)
            NodeKindsVariant {
                name: "SKIPPED".to_owned(),
                documentation:
                    "Used to hold parts of input that cannot be parsed (incomplete, or erroneous)."
                        .to_owned(),
            },
        );

        return Self {
            type_name: "TokenKind".to_string(),
            variants,
        };
    }
}

fn collect_top_level(
    language: &LanguageDefinitionRef,
    production: &ProductionRef,
    variants: &mut Vec<NodeKindsVariant>,
) {
    if production.inlined {
        return;
    }

    let documentation = extract_documentation(language, production);

    variants.push(NodeKindsVariant {
        name: production.name.to_owned(),
        documentation: documentation[&production.name].to_owned(),
    });
}

fn collect_recursive(
    language: &LanguageDefinitionRef,
    production: &ProductionRef,
    variants: &mut Vec<NodeKindsVariant>,
) {
    let documentation = extract_documentation(language, production);

    if !production.inlined {
        variants.push(NodeKindsVariant {
            name: production.name.to_owned(),
            documentation: documentation[&production.name].to_owned(),
        });
    }

    let version_map = match &production.definition {
        ProductionDefinition::PrecedenceParser { version_map, .. } => version_map,
        ProductionDefinition::Parser { .. }
        | ProductionDefinition::Scanner { .. }
        | ProductionDefinition::TriviaParser { .. } => {
            // Nothing recursive to collect
            return;
        }
    };

    let expressions: Vec<_> = match &version_map {
        codegen_schema::types::VersionMap::Unversioned(unversioned) => unversioned
            .operator_expressions
            .iter()
            .map(|expression| &expression.name)
            .unique()
            .collect(),
        codegen_schema::types::VersionMap::Versioned(versioned) => versioned
            .values()
            .filter_map(|version| version.as_ref())
            .flat_map(|precedence_parser| &precedence_parser.operator_expressions)
            .map(|expression| &expression.name)
            .unique()
            .collect(),
    };

    for expression in expressions {
        variants.push(NodeKindsVariant {
            name: expression.to_owned(),
            documentation: documentation[expression].to_owned(),
        });
    }
}

// Constructs all documentation headers for the given production, including any sub-statements
// It returns a map from each statement name to its markdown documentation:
fn extract_documentation(
    language: &LanguageDefinitionRef,
    production: &ProductionRef,
) -> HashMap<String, String> {
    let mut results = HashMap::<String, Vec<String>>::new();

    match production.versions() {
        Some(versions) => {
            for version in versions {
                if let Some(outputs) =
                    EbnfSerializer::serialize_version(language, production, version)
                {
                    for (name, ebnf) in outputs {
                        results
                            .entry(name)
                            .or_default()
                            .extend([format!("## v{version}"), format!("```ebnf\n{ebnf}\n```")]);
                    }
                } else {
                    results
                        .entry(production.name.to_owned())
                        .or_default()
                        .extend([
                            format!("## v{version}"),
                            format!("```ebnf\n(* DELETED *)\n```"),
                        ]);
                }
            }
        }
        None => {
            let latest_version = language.versions.last().unwrap();
            let outputs =
                EbnfSerializer::serialize_version(language, production, latest_version).unwrap();

            for (name, ebnf) in outputs {
                results
                    .entry(name)
                    .or_default()
                    .extend([format!("## Unversioned"), format!("```ebnf\n{ebnf}\n```")]);
            }
        }
    };

    return results
        .into_iter()
        .map(|(key, entries)| (key, entries.join("\n\n")))
        .collect();
}
