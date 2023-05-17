use codegen_utils::errors::CodegenResult;
use semver::Version;

use crate::{
    types::{ProductionRef, SchemaRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub fn run(schema: &SchemaRef) -> CodegenResult<()> {
    let mut visitor = LanguageVersions::new(schema);
    let mut reporter = Reporter::new();

    run_visitor(&mut visitor, schema, &mut reporter);

    return reporter.to_result();
}

struct LanguageVersions {
    schema: SchemaRef,
}

impl LanguageVersions {
    fn new(schema: &SchemaRef) -> Self {
        return Self {
            schema: schema.to_owned(),
        };
    }
}

impl Visitor for LanguageVersions {
    fn visit_manifest(&mut self, location: &LocationRef, reporter: &mut Reporter) -> bool {
        if self.schema.versions.is_empty() {
            reporter.report(&location, Errors::Empty);
            return false;
        }

        for window in self.schema.versions.windows(2) {
            let current = &window[0];
            let next = &window[1];

            if current >= next {
                reporter.report(&location, Errors::NotSorted(current.to_owned()));
                return false;
            }
        }

        return true;
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
            reporter.report(&location, Errors::Empty);
            return false;
        }

        for window in versions.windows(2) {
            let current = window[0];
            let next = window[1];

            if current >= next {
                reporter.report(&location, Errors::NotSorted(current.to_owned()));
                return false;
            }
        }

        for version in versions {
            if !self.schema.versions.contains(version) {
                reporter.report(location, Errors::Unknown(version.to_owned()));
            }
        }

        return false;
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
