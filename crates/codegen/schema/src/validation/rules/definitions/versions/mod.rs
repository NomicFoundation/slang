use semver::Version;

use crate::{
    types::{LanguageDefinitionRef, ProductionRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub struct Versions {
    language: LanguageDefinitionRef,
}

impl Versions {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let mut instance = Self {
            language: language.to_owned(),
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Versions {
    fn visit_manifest(&mut self, location: &LocationRef, reporter: &mut Reporter) -> bool {
        if self.language.versions.is_empty() {
            reporter.report(location, Errors::Empty);
            return false;
        }

        for window in self.language.versions.windows(2) {
            let current = &window[0];
            let next = &window[1];

            if current >= next {
                reporter.report(location, Errors::NotSorted(current.to_owned()));
                return false;
            }
        }

        true
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        let versions = match production.versions() {
            Some(versions) => versions,
            None => return false,
        };

        if versions.is_empty() {
            reporter.report(location, Errors::Empty);
            return false;
        }

        for window in versions.windows(2) {
            let current = window[0];
            let next = window[1];

            if current >= next {
                reporter.report(location, Errors::NotSorted(current.to_owned()));
                return false;
            }
        }

        for version in versions {
            if !self.language.versions.contains(version) {
                reporter.report(location, Errors::Unknown(version.to_owned()));
            }
        }

        false
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Must define at least one version.")]
    Empty,
    #[error("Version '{0}' must be sorted with no duplicates (strictly increasing).")]
    NotSorted(Version),
    #[error("Version '{0}' does not exist in the manifest.")]
    Unknown(Version),
}
