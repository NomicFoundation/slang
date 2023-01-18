use std::collections::HashSet;

use semver::Version;

use crate::{
    grammar::Grammar,
    manifest::{
        EBNFDelimitedBy, EBNFRepeat, EBNFSeparatedBy, Expression, ProductionVersioning, EBNF,
    },
};

static REQUIRED_PRODUCTIONS: [&str; 2] = ["LeadingTrivia", "TrailingTrivia"];

pub fn validate_grammar(grammar: &Grammar) {
    validate_versions_list(
        "Grammar Manifest",
        &grammar.manifest.versions.iter().collect(),
    );

    let defined = validate_definitions(grammar);
    let used = validate_usages(grammar, &defined);
    validate_orphaned_nodes(&grammar, defined, used);
}

fn validate_versions_list(owner: &str, versions: &Vec<&Version>) {
    assert_ne!(
        versions.len(),
        0,
        "{owner}: must define at least one version"
    );

    let mut expected = versions.clone();
    expected.sort();
    expected.dedup();

    assert_eq!(
        versions, &expected,
        "{owner}: versions to be sorted and unique."
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
        productions
            .iter()
            .for_each(|production| match &production.versioning {
                ProductionVersioning::Unversioned(expression) => {
                    validate_expression(&expression, defined, &mut used);
                }
                ProductionVersioning::Versioned(versions) => {
                    let production_name = &&production.name;
                    validate_versions_list(production_name, &versions.keys().collect());

                    for (version, expression) in versions {
                        assert!(
                            grammar.manifest.versions.contains(&version),
                            "{production_name}: Version {version} is not defined in the manifest.",
                        );

                        validate_expression(&expression, defined, &mut used);
                    }
                }
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

        EBNF::DelimitedBy(EBNFDelimitedBy { expression, .. })
        | EBNF::Not(expression)
        | EBNF::OneOrMore(expression)
        | EBNF::Optional(expression)
        | EBNF::Repeat(EBNFRepeat { expression, .. })
        | EBNF::SeparatedBy(EBNFSeparatedBy { expression, .. })
        | EBNF::ZeroOrMore(expression) => {
            validate_expression(expression, defined, used);
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
