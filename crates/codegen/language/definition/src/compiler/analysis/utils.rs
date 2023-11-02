use crate::{compiler::analysis::Analysis, model::spanned::VersionSpecifier, utils::VersionSet};

impl Analysis {
    pub fn add_specifier(&self, set: &mut VersionSet, specifier: &VersionSpecifier) {
        match specifier {
            VersionSpecifier::Never => {
                // Do nothing.
            }
            VersionSpecifier::From { from } => {
                set.add_versions_starting_from(from);
            }
            VersionSpecifier::Till { till } => {
                set.add_version_range(&self.language.versions[0], till);
            }
            VersionSpecifier::Range { from, till } => {
                set.add_version_range(from, till);
            }
        }
    }

    pub fn add_all_versions(&self, set: &mut VersionSet) {
        set.add_versions_starting_from(&self.language.versions[0]);
    }
}
