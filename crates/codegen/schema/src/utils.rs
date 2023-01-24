use std::collections::BTreeSet;

use semver::Version;

use crate::types::{grammar::Grammar, productions::ProductionVersioning};

impl Grammar {
    pub fn collect_version_breaks<'a>(&'a self) -> BTreeSet<&'a Version> {
        let mut version_breaks = BTreeSet::new();
        version_breaks.insert(self.versions.first().unwrap());
        version_breaks.insert(self.versions.last().unwrap());

        for production in self.productions.values() {
            match &production.versioning {
                ProductionVersioning::Unversioned(_) => {}
                ProductionVersioning::Versioned(versions) => {
                    version_breaks.extend(versions.keys());
                }
            }
        }

        return version_breaks;
    }
}
