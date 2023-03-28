use std::{collections::HashSet, path::PathBuf};

use codegen_utils::errors::CodegenErrors;
use semver::Version;

use crate::{
    types::manifest::Manifest,
    types::production::ProductionRef,
    visitor::{Visitor, VisitorResponse},
    yaml::cst::NodeRef,
};

pub struct Validator<'ce> {
    errors: &'ce CodegenErrors,
    manifest_versions: HashSet<Version>,
}

impl<'ce> Validator<'ce> {
    pub fn new(errors: &'ce mut CodegenErrors) -> Self {
        return Self {
            errors,
            manifest_versions: Default::default(),
        };
    }
}

impl Visitor for Validator<'_> {
    fn visit_manifest(
        &mut self,
        manifest: &Manifest,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        self.manifest_versions.extend(manifest.versions.clone());
        self.check_order(path, node, &manifest.versions);

        return VisitorResponse::StepOut;
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        if let Some(versions) = production.versions() {
            let node = node.value_of_field("versioned");
            for version in versions.iter() {
                if !self.manifest_versions.contains(version) {
                    self.errors.push(
                        path,
                        node.key_of_field(&version.to_string()).range(),
                        Errors::Unknown(version.clone()),
                    );
                }
            }
            self.check_order(path, &node, &versions);
        }

        return VisitorResponse::StepOut;
    }
}

impl Validator<'_> {
    fn check_order(&self, path: &PathBuf, node: &NodeRef, versions: &Vec<Version>) {
        if versions.is_empty() {
            self.errors.push(path, node.range(), Errors::Empty);
            return;
        }

        for window in versions.windows(2) {
            if let [current, next] = window {
                if current >= next {
                    self.errors.push(path, node.range(), Errors::NotSorted);
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
    #[error("At least one version is required.")]
    Empty,
    #[error("Versions must occur in strictly increasing order.")]
    NotSorted,
    #[error("Version '{0}' does not exist in the manifest.")]
    Unknown(Version),
}
