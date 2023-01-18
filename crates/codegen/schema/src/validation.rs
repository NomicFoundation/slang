use std::collections::{HashMap, HashSet};

use codegen_utils::errors::CodegenResult;
use semver::Version;

use crate::{
    types::{
        manifest::ManifestFile,
        productions::{
            EBNFDelimitedBy, EBNFRepeat, EBNFSeparatedBy, Expression, ProductionRef,
            ProductionVersioning, EBNF,
        },
    },
    yaml::files::File,
};

pub fn check_manifest(manifest_file: &File<ManifestFile>) -> CodegenResult<()> {
    validate_versions_list(
        "Grammar Manifest",
        &manifest_file.value.versions.iter().collect(),
    );

    return Ok(());
}

pub fn check_productions(
    manifest_file: &File<ManifestFile>,
    productions: &HashMap<String, ProductionRef>,
) -> CodegenResult<()> {
    let required = &HashSet::from([
        manifest_file.value.root_production.to_owned(),
        "LeadingTrivia".to_string(),
        "TrailingTrivia".to_string(),
    ]);

    let defined = collect_definitions(&required, &productions);
    let used = collect_usages(&productions, &defined, &manifest_file.value.versions);
    validate_orphaned_nodes(&required, &defined, &used);

    return Ok(());
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

fn collect_definitions(
    required: &HashSet<String>,
    productions: &HashMap<String, ProductionRef>,
) -> HashSet<String> {
    let mut defined = HashSet::<String>::new();

    productions.keys().for_each(|production| {
        assert!(
            defined.insert(production.clone()),
            "Production '{production}' defined more than once",
        );
    });

    for production_name in required {
        assert!(
            defined.contains(production_name),
            "Grammar must contain a '{}' production",
            production_name
        );
    }

    return defined;
}

fn collect_usages(
    productions: &HashMap<String, ProductionRef>,
    defined: &HashSet<String>,
    manifest_versions: &Vec<Version>,
) -> HashSet<String> {
    let mut used = HashSet::<String>::new();

    productions
        .values()
        .for_each(|production| match &production.versioning {
            ProductionVersioning::Unversioned(expression) => {
                validate_expression(&expression, defined, &mut used);
            }
            ProductionVersioning::Versioned(versions) => {
                let production_name = &&production.name;
                validate_versions_list(production_name, &versions.keys().collect());

                for (version, expression) in versions {
                    assert!(
                        manifest_versions.contains(&version),
                        "{production_name}: Version {version} is not defined in the manifest.",
                    );

                    validate_expression(&expression, defined, &mut used);
                }
            }
        });

    return used;
}

fn validate_orphaned_nodes(
    required: &HashSet<String>,
    defined: &HashSet<String>,
    used: &HashSet<String>,
) {
    for name in defined {
        if !required.contains(name) {
            assert!(
                used.contains(name),
                "Production '{}' is defined but not referenced anywhere",
                name
            );
        }
    }
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
