use std::collections::BTreeSet;

use codegen_schema::types::{grammar::Grammar, productions::ProductionVersioning};
use semver::Version;

pub fn collect_breaking_versions<'a>(grammar: &'a Grammar) -> BTreeSet<&'a Version> {
    let mut breaking_versions = BTreeSet::from([
        grammar.versions.first().unwrap(),
        grammar.versions.last().unwrap(),
    ]);

    for production in grammar.productions.values() {
        match &production.versioning {
            ProductionVersioning::Unversioned(_) => {
                // Nothing to add
            }
            ProductionVersioning::Versioned(versions) => {
                for version in versions.keys() {
                    breaking_versions.insert(version);
                }
            }
        }
    }

    return breaking_versions;
}
