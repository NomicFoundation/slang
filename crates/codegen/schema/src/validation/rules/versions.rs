use std::collections::HashSet;

use codegen_utils::errors::CodegenErrors;
use semver::Version;

use crate::{
    validation::{
        ast::{
            files::ManifestFile,
            node::Node,
            production::ProductionRef,
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
    yaml::cst,
};

pub fn check(model: &Model, errors: &mut CodegenErrors) {
    let mut visitor = VersionsChecker::new(model);

    visitor.visit(model, errors);
}

struct VersionsChecker<'v> {
    manifest_versions: HashSet<&'v Version>,
}

impl<'v> VersionsChecker<'v> {
    fn new(model: &'v Model) -> Self {
        let manifest_versions = model
            .manifest_file
            .ast
            .value
            .versions
            .iter()
            .map(|version| &version.value)
            .collect();

        return Self { manifest_versions };
    }
}

impl Visitor for VersionsChecker<'_> {
    fn visit_manifest(
        &mut self,
        manifest_file: &ManifestFile,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        let manifest = &manifest_file.ast.value;

        self.check_order(
            &manifest.title.cst_node,
            &manifest.versions.iter().collect(),
            reporter,
        );

        return VisitorResponse::StepOut;
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        if let Some(versions) = production.versions() {
            for version in &versions {
                let Node { cst_node, value } = version;
                if !self.manifest_versions.contains(value) {
                    reporter.report(cst_node, Errors::Unknown(value.to_owned()));
                }
            }
            self.check_order(&production.version_map_cst_node(), &versions, reporter);
        }

        return VisitorResponse::StepOut;
    }
}

impl<'v> VersionsChecker<'v> {
    fn check_order(
        &self,
        parent_node: &cst::NodeRef,
        versions: &Vec<&Node<Version>>,
        reporter: &mut Reporter,
    ) {
        if versions.is_empty() {
            reporter.report(&parent_node, Errors::Empty);
            return;
        }

        for window in versions.windows(2) {
            if let [current, next] = window {
                if current.value >= next.value {
                    reporter.report(&next.cst_node, Errors::NotSorted);
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
