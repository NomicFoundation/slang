use std::collections::BTreeSet;

use codegen_schema::types::{LanguageDefinitionRef, ProductionDefinition};
use quote::{format_ident, quote};

pub fn generate_production_kind(language: &LanguageDefinitionRef) -> String {
    let mut kinds = vec![];

    for production in language.productions.values() {
        if production.inlined {
            continue;
        }

        kinds.push(production.name.as_str());
    }

    kinds.sort();

    return generate_enum_file("ProductionKind", kinds.into_iter());
}

pub fn generate_rule_kind(language: &LanguageDefinitionRef) -> String {
    // Use a BTreeSet to deduplicate operator names:
    let mut kinds = BTreeSet::new();

    for production in language.productions.values() {
        if production.inlined {
            continue;
        }

        match &production.definition {
            ProductionDefinition::Parser { .. } | ProductionDefinition::TriviaParser { .. } => {
                kinds.insert(production.name.as_str());
            }
            ProductionDefinition::PrecedenceParser { version_map, .. } => {
                kinds.insert(production.name.as_str());

                match version_map {
                    codegen_schema::types::VersionMap::Unversioned(unversioned) => {
                        for expression in &unversioned.operator_expressions {
                            kinds.insert(expression.name.as_str());
                        }
                    }
                    codegen_schema::types::VersionMap::Versioned(versioned) => {
                        for version in versioned.values() {
                            if let Some(version) = version {
                                for expression in &version.operator_expressions {
                                    kinds.insert(expression.name.as_str());
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

    return generate_enum_file("RuleKind", kinds.into_iter());
}

pub fn generate_token_kind(language: &LanguageDefinitionRef) -> String {
    let mut kinds = vec![];

    for production in language.productions.values() {
        if production.inlined {
            continue;
        }

        match production.definition {
            ProductionDefinition::Scanner { .. } => {
                kinds.push(production.name.as_str());
            }
            ProductionDefinition::Parser { .. }
            | ProductionDefinition::PrecedenceParser { .. }
            | ProductionDefinition::TriviaParser { .. } => {}
        };
    }

    kinds.sort();

    // Insert "SKIPPED" as the first token kind (after sorting):
    kinds.insert(0, "SKIPPED");

    return generate_enum_file("TokenKind", kinds.into_iter());
}

fn generate_enum_file<'context, I>(type_name: &str, variants: I) -> String
where
    I: Iterator<Item = &'context str>,
{
    let type_name = format_ident!("{type_name}");
    let variants = variants.map(|variant| format_ident!("{variant}"));

    let enum_file = quote! {
        #[cfg(feature = "slang_napi_interfaces")]
        use napi::bindgen_prelude::*;

        #[derive(
            Debug,
            Eq,
            Ord,
            PartialEq,
            PartialOrd,
            serde::Serialize,
            strum_macros::AsRefStr,
            strum_macros::Display,
            strum_macros::EnumString,
        )]
        #[cfg_attr(
            // If feature is enabled, derive the NAPI version.
            // This also derives `Clone` and `Copy` automatically.
            feature = "slang_napi_interfaces",
            napi(string_enum, namespace = "syntax$nodes")
        )]
        #[cfg_attr(
            // If feature is not enabled, derive `Clone` and `Copy` manually.
            not(feature = "slang_napi_interfaces"),
            derive(Clone, Copy),
        )]
        pub enum #type_name {
            #(#variants),*
        }
    };

    return enum_file.to_string();
}
