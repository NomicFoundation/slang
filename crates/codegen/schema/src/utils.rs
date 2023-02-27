use std::collections::BTreeSet;

use semver::Version;

use crate::types::grammar::Grammar;

impl Grammar {
    pub fn collect_version_breaks<'a>(&'a self) -> BTreeSet<Version> {
        let mut version_breaks = BTreeSet::new();
        version_breaks.insert(self.versions.first().cloned().unwrap());
        version_breaks.insert(self.versions.last().cloned().unwrap());

        for production in self.productions.values() {
            if let Some(versions) = production.versions() {
                version_breaks.extend(versions.into_iter());
            }
        }

        return version_breaks;
    }
}
