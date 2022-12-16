use std::collections::BTreeSet;

use codegen_schema::{Grammar, ProductionVersions};
use semver::Version;

pub fn collect_breaking_versions<'a>(grammar: &'a Grammar) -> BTreeSet<&'a Version> {
    let mut breaking_versions = BTreeSet::from([
        grammar.manifest.versions.first().unwrap(),
        grammar.manifest.versions.last().unwrap(),
    ]);

    for production in grammar.productions.values().flatten() {
        match &production.versions {
            ProductionVersions::Unversioned(_) => {
                // Nothing to add
            }
            ProductionVersions::Versioned(versions) => {
                for version in versions.keys() {
                    breaking_versions.insert(version);
                }
            }
        }
    }

    return breaking_versions;
}
