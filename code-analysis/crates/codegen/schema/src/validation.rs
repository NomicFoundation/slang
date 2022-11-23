use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use codegen_utils::context::CodegenContext;
use jsonschema::JSONSchema;
use regex::Regex;
use semver::Version;

use crate::{EBNFDelimitedBy, EBNFRepeat, EBNFSeparatedBy, Expression, Grammar, EBNF};

static REQUIRED_PRODUCTIONS: [&str; 2] = ["LeadingTrivia", "TrailingTrivia"];

impl Grammar {
    pub fn validate(&self, context: &CodegenContext, manifest_path: &PathBuf) {
        validate_topics(self);
        validate_versions_list(&self.manifest.versions);

        let mut schemas = LoadedSchemas::new();
        schemas.validate(context, manifest_path);
        self.productions.keys().for_each(|definition| {
            schemas.validate(
                context,
                &manifest_path
                    .parent()
                    .unwrap()
                    .join(definition)
                    .canonicalize()
                    .unwrap(),
            );
        });

        let defined = validate_definitions(self);
        let used = validate_usages(self, &defined);
        validate_orphaned_nodes(&self, defined, used);
    }
}

fn validate_topics(grammar: &Grammar) {
    grammar
        .manifest
        .sections
        .iter()
        .enumerate()
        .for_each(|(section_index, section)| {
            section
                .topics
                .iter()
                .enumerate()
                .for_each(|(topic_index, topic)| {
                    let slug = grammar.generate_topic_slug(section_index, topic_index);

                    if let Some(topic_definition) = &topic.definition {
                        assert_eq!(topic_definition, &format!("topics/{slug}.yml"));
                    }
                });
        });
}

fn validate_versions_list(versions: &Vec<Version>) {
    assert_ne!(versions.len(), 0, "Grammar must have at least one version");

    let mut expected = versions.clone();
    expected.sort();
    expected.dedup();

    assert_eq!(
        versions, &expected,
        "Expected manifest versions to be sorted and unique."
    );
}

fn validate_definitions(grammar: &Grammar) -> HashSet<String> {
    let mut defined = HashSet::<String>::new();

    grammar.productions.values().for_each(|productions| {
        productions.iter().for_each(|production| {
            assert!(
                defined.insert(production.name.clone()),
                "Production '{}' defined more than once",
                production.name
            );
        });
    });

    assert!(
        defined.contains(&grammar.manifest.root_production),
        "Root production '{}' is not defined in grammar",
        grammar.manifest.root_production
    );

    for production_name in REQUIRED_PRODUCTIONS {
        assert!(
            defined.contains(production_name),
            "Grammar must contain a '{}' production",
            production_name
        );
    }

    return defined;
}

fn validate_usages(grammar: &Grammar, defined: &HashSet<String>) -> HashSet<String> {
    let mut used = HashSet::<String>::new();

    grammar.productions.values().for_each(|productions| {
        productions.iter().for_each(|production| {
            production
                .versions
                .iter()
                .for_each(|(version, expression)| {
                    validate_version(grammar, version);
                    validate_expression(expression, defined, &mut used);
                });
        });
    });

    return used;
}

fn validate_orphaned_nodes(grammar: &Grammar, defined: HashSet<String>, used: HashSet<String>) {
    defined.iter().for_each(|name| {
        if name != &grammar.manifest.root_production
            && !REQUIRED_PRODUCTIONS.contains(&name.as_str())
        {
            assert!(
                used.contains(name),
                "Production '{}' is defined but not referenced anywhere",
                name
            );
        }
    });
}

fn validate_expression(
    expression: &Expression,
    defined: &HashSet<String>,
    used: &mut HashSet<String>,
) {
    match &expression.ebnf {
        EBNF::Choice(elements) | EBNF::Sequence(elements) => {
            elements.iter().for_each(|sub_expression| {
                validate_expression(sub_expression, defined, used);
            });
        }

        EBNF::DelimitedBy(EBNFDelimitedBy { expr, .. })
        | EBNF::Not(expr)
        | EBNF::OneOrMore(expr)
        | EBNF::Optional(expr)
        | EBNF::Repeat(EBNFRepeat { expr, .. })
        | EBNF::SeparatedBy(EBNFSeparatedBy { expr, .. })
        | EBNF::ZeroOrMore(expr) => {
            validate_expression(expr, defined, used);
        }

        EBNF::Difference(difference) => {
            validate_expression(&difference.minuend, defined, used);
            validate_expression(&difference.subtrahend, defined, used);
        }

        EBNF::Range(..) | EBNF::Terminal(..) => {}

        EBNF::Reference(reference) => {
            assert!(
                defined.contains(reference),
                "Production '{}' referenced but not defined in grammar",
                reference
            );

            used.insert(reference.clone());
        }
    };
}

fn validate_version(grammar: &Grammar, version: &Version) {
    if version == &Version::new(0, 0, 0) {
        return;
    }

    assert!(
        grammar.manifest.versions.contains(&version),
        "Version {version} is not defined in the manifest.",
    );
}

struct LoadedSchemas {
    schemas: HashMap<PathBuf, JSONSchema>,
}

impl LoadedSchemas {
    pub fn new() -> Self {
        return Self {
            schemas: HashMap::new(),
        };
    }

    pub fn validate(&mut self, context: &CodegenContext, yaml_path: &PathBuf) {
        let yaml_contents = context.read_file(&yaml_path).unwrap();

        let schema_path_re = Regex::new(r"# yaml-language-server: \$schema=([\.\-/a-z]+)").unwrap();
        let mut schema_path_matches = schema_path_re.captures_iter(&yaml_contents);
        let schema_path = schema_path_matches.next().unwrap().get(1).unwrap().as_str();
        assert!(
            schema_path_matches.next().is_none(),
            "Multiple schema paths"
        );

        let schema_path = yaml_path
            .parent()
            .unwrap()
            .join(schema_path)
            .canonicalize()
            .unwrap();

        if !self.schemas.contains_key(&schema_path) {
            let schema_contents = context.read_file(&schema_path).unwrap();
            let schema_json = &serde_json::from_str(&schema_contents).unwrap();
            let schema = JSONSchema::compile(schema_json).unwrap();

            self.schemas.insert(schema_path.to_owned(), schema);
        }

        let schema = self.schemas.get(&schema_path).unwrap();

        let yaml_value = serde_yaml::from_str(&yaml_contents).unwrap();
        let json_value = serde_json::from_value(yaml_value).unwrap();

        let validation = schema.validate(&json_value);
        if let Err(errors) = validation {
            for error in errors {
                println!("{:?}", error);
            }

            panic!("Schema Validation errors in: {:?}", yaml_path);
        }
    }
}
