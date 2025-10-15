use crate::compiler::analysis::Analysis;
use crate::compiler::version_set::VersionSet;
use crate::model::SpannedVersionSpecifier;

impl Analysis {
    pub fn add_specifier(&self, set: &mut VersionSet, specifier: &SpannedVersionSpecifier) {
        match specifier {
            SpannedVersionSpecifier::Never => {
                // Do nothing.
            }
            SpannedVersionSpecifier::From { from } => {
                set.add_versions_starting_from(from);
            }
            SpannedVersionSpecifier::Till { till } => {
                set.add_version_range(&self.language.versions[0], till);
            }
            SpannedVersionSpecifier::Range { from, till } => {
                set.add_version_range(from, till);
            }
        }
    }

    pub fn add_all_versions(&self, set: &mut VersionSet) {
        set.add_versions_starting_from(&self.language.versions[0]);
    }
}
