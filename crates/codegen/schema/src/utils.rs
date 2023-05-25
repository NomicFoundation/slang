use std::collections::BTreeSet;

use semver::Version;

use crate::types::LanguageDefinition;

impl LanguageDefinition {
    pub fn collect_version_breaks(&self) -> BTreeSet<Version> {
        let mut version_breaks = BTreeSet::new();
        version_breaks.insert(self.versions.first().cloned().unwrap());

        for production in self.productions.values() {
            if let Some(versions) = production.versions() {
                version_breaks.extend(versions.into_iter().cloned());
            }
        }

        return version_breaks;
    }
}
