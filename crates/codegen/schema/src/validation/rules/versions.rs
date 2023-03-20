use std::collections::HashSet;

use semver::Version;

use crate::{
    types::manifest::Manifest,
    types::production::ProductionRef,
    validation::Model,
    visitor::{Visitor, VisitorResponse},
    yaml::cst,
};

use super::Reporter;

pub fn check(model: &Model, reporter: Reporter) {
    let mut visitor = VersionsChecker::new(model, reporter);
    visitor.visit(model);
}

struct VersionsChecker<'r> {
    manifest_versions: HashSet<Version>,
    reporter: Reporter<'r>,
}

impl<'r> VersionsChecker<'r> {
    fn new(model: &Model, reporter: Reporter) -> Self {
        let manifest_versions = model
            .manifest_file
            .ast
            .value
            .versions
            .iter()
            .map(|version| version.value.clone())
            .collect();

        return Self {
            manifest_versions,
            reporter,
        };
    }
}

impl Visitor for VersionsChecker<'_> {
    fn visit_manifest(&mut self, manifest: &Manifest) -> VisitorResponse {
        self.check_order(
            &manifest.title.cst_node,
            &manifest.versions.iter().collect(),
        );

        return VisitorResponse::StepOut;
    }

    fn visit_production(&mut self, production: &ProductionRef) -> VisitorResponse {
        if let Some(versions) = production.versions() {
            for version in &versions {
                let Node { cst_node, value } = version;
                if !self.manifest_versions.contains(value) {
                    self.reporter
                        .report(cst_node, Errors::Unknown(value.to_owned()));
                }
            }
            self.check_order(&production.version_map_cst_node(), &versions);
        }

        return VisitorResponse::StepOut;
    }
}

impl VersionsChecker<'_> {
    fn check_order(&self, parent_node: &cst::NodeRef, versions: &Vec<&Node<Version>>) {
        if versions.is_empty() {
            self.reporter.report(&parent_node, Errors::Empty);
            return;
        }

        for window in versions.windows(2) {
            if let [current, next] = window {
                if current.value >= next.value {
                    self.reporter.report(&next.cst_node, Errors::NotSorted);
                    break;
                }
            } else {
                unreachable!();
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Must define at least one version.")]
    Empty,
    #[error("Versions must be sorted with no duplicates (strictly increasing).")]
    NotSorted,
    #[error("Version '{0}' does not exist in the manifest.")]
    Unknown(Version),
}
