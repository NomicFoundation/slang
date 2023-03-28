use std::{collections::HashSet, path::PathBuf};

use codegen_utils::errors::CodegenErrors;

use crate::{
    types::parser::ParserDefinition,
    types::scanner::ScannerDefinition,
    validation::model::Model,
    visitor::{Visitor, VisitorResponse},
    yaml::cst::{self, NodeRef},
};

use super::definitions::Definitions;

pub fn check(model: &Model, definitions: &Definitions, errors: &mut CodegenErrors) {
    let mut collector = Validator::new(definitions);

    collector.visit(model, errors);
    collector.check_unused(errors);
}

struct Validator<'v> {
    definitions: &'v Definitions,
    references: HashSet<String>,
}

impl<'v> Validator<'v> {
    fn new(definitions: &'v Definitions) -> Self {
        return Self {
            definitions,
            references: HashSet::new(),
        };
    }
}

impl Visitor for Validator<'_> {
    fn visit_scanner_definition(
        &mut self,
        scanner: &ScannerDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        if let ScannerDefinition::Reference(reference) = scanner {
            let Node { cst_node, value } = reference;
            self.check_is_scanner_reference(cst_node, value, reporter);
            return VisitorResponse::StepOut;
        } else {
            return VisitorResponse::StepIn;
        }
    }

    fn visit_parser_definition(
        &mut self,
        definition: &ParserDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        match definition {
            ParserDefinition::Reference(reference) => {
                let Node { cst_node, value } = reference;
                self.check_is_scanner_or_parser_reference(cst_node, value, reporter);
                return VisitorResponse::StepOut;
            }
            ParserDefinition::DelimitedBy { open, close, .. } => {
                let Node { cst_node, value } = &open.value.reference;
                self.check_is_scanner_reference(cst_node, value, reporter);
                let Node { cst_node, value } = &close.value.reference;
                self.check_is_scanner_reference(cst_node, value, reporter);
            }
            ParserDefinition::SeparatedBy {
                separator: reference,
                ..
            }
            | ParserDefinition::TerminatedBy {
                terminator: reference,
                ..
            } => {
                let Node { cst_node, value } = &reference.value.reference;
                self.check_is_scanner_reference(cst_node, value, reporter);
            }
            _ => {}
        }
        return VisitorResponse::StepIn;
    }
}

impl<'v> Validator<'v> {
    fn check_unused(&self, errors: &mut CodegenErrors) {
        for (name, production) in &self.definitions.top_level_parsers {
            if self.definitions.required.contains(name) {
                continue;
            }

            if !self.references.contains(name) {
                errors.push(
                    &production.path,
                    production.cst_node.range(),
                    Errors::UnusedParser(name.to_owned()),
                );
            }
        }

        for (name, production) in &self.definitions.top_level_scanners {
            if self.definitions.required.contains(name) {
                continue;
            }

            if !self.references.contains(name) {
                errors.push(
                    &production.path,
                    production.cst_node.range(),
                    Errors::UnusedScanner(name.to_owned()),
                );
            }
        }
    }

    fn check_is_scanner_reference(
        &mut self,
        cst_node: &cst::Node,
        value: &String,
        reporter: &mut Reporter,
    ) {
        if self.definitions.top_level_scanners.contains_key(value) {
            self.references.insert(value.clone());
        } else if self.definitions.top_level_parsers.contains_key(value) {
            reporter.report(cst_node, Errors::MustBeAScanner(value.clone()));
        } else if self.definitions.internal_named_parsers.contains_key(value) {
            reporter.report(
                cst_node,
                Errors::InnerNodesCannotBeReferenced(value.clone()),
            );
        } else {
            reporter.report(cst_node, Errors::NoSuchProduction(value.to_owned()));
        }
    }

    fn check_is_scanner_or_parser_reference(
        &mut self,
        cst_node: &cst::Node,
        value: &String,
        reporter: &mut Reporter,
    ) {
        if self.definitions.top_level_scanners.contains_key(value) {
            self.references.insert(value.clone());
        } else if self.definitions.top_level_parsers.contains_key(value) {
            self.references.insert(value.clone());
        } else if self.definitions.internal_named_parsers.contains_key(value) {
            reporter.report(
                cst_node,
                Errors::InnerNodesCannotBeReferenced(value.clone()),
            );
        } else {
            reporter.report(cst_node, Errors::NoSuchProduction(value.to_owned()));
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("There is no Production '{0}'.")]
    NoSuchProduction(String),
    #[error("Production '{0}' must be a Scanner.")]
    MustBeAScanner(String),
    #[error("Cannot reference inner node '{0}', only top-level productions.")]
    InnerNodesCannotBeReferenced(String),
    #[error("Parser '{0}' is not used.")]
    UnusedParser(String),
    #[error("Scanner '{0}' is not used.")]
    UnusedScanner(String),
}
