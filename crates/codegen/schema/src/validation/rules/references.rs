use std::collections::HashSet;

use codegen_utils::errors::CodegenErrors;

use crate::validation::{
    ast::{
        node::Node,
        parser::{ParserDefinition, ParserRef},
        precedence_parser::Reference,
        scanner::{ScannerDefinition, ScannerRef},
        visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
    },
    Model,
};

use super::definitions::Definitions;

pub fn check(model: &Model, definitions: &Definitions, errors: &mut CodegenErrors) {
    let mut collector = ReferencesCollector::new(definitions);

    collector.visit(model, errors);
    collector.check_unused(errors);
}

struct ReferencesCollector<'v> {
    definitions: &'v Definitions,
    references: HashSet<String>,
}

impl<'v> ReferencesCollector<'v> {
    fn new(definitions: &'v Definitions) -> Self {
        return Self {
            definitions,
            references: HashSet::new(),
        };
    }
}

impl Visitor for ReferencesCollector<'_> {
    fn visit_scanner(&mut self, scanner: &ScannerRef, reporter: &mut Reporter) -> VisitorResponse {
        if let ScannerDefinition::Reference(reference) = &scanner.definition.value {
            let Node { cst_node, value } = reference;
            if self.definitions.internal_named_parsers.contains_key(value) {
                reporter.report(cst_node, Errors::NodesCannotBeReferenced(value.to_owned()))
            } else if self.definitions.top_level_parsers.contains_key(value) {
                reporter.report(
                    cst_node,
                    Errors::ScannersCanOnlyReferenceScanners(value.to_owned()),
                )
            } else if !self.definitions.top_level_scanners.contains_key(value) {
                reporter.report(cst_node, Errors::NoSuchProduction(value.to_owned()))
            } else {
                self.references.insert(reference.value.to_owned());
            }

            return VisitorResponse::StepOut;
        } else {
            return VisitorResponse::StepIn;
        }
    }

    fn visit_parser(&mut self, parser: &ParserRef, reporter: &mut Reporter) -> VisitorResponse {
        if let ParserDefinition::Reference(reference) = &parser.definition.value {
            let Node { cst_node, value } = reference;
            if self.definitions.internal_named_parsers.contains_key(value) {
                reporter.report(cst_node, Errors::NodesCannotBeReferenced(value.to_owned()))
            } else if !self.definitions.top_level_parsers.contains_key(value)
                && !self.definitions.top_level_scanners.contains_key(value)
            {
                reporter.report(cst_node, Errors::NoSuchProduction(value.to_owned()))
            } else {
                self.references.insert(reference.value.to_owned());
            }

            return VisitorResponse::StepOut;
        } else {
            return VisitorResponse::StepIn;
        }
    }

    fn visit_primary_expression_reference(
        &mut self,
        reference: &Reference,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        let Node { cst_node, value } = &reference.reference;
        if self.definitions.internal_named_parsers.contains_key(value) {
            reporter.report(cst_node, Errors::NodesCannotBeReferenced(value.to_owned()))
        } else if !self.definitions.top_level_parsers.contains_key(value) {
            reporter.report(cst_node, Errors::NoSuchPrimaryExpression(value.to_owned()))
        } else {
            self.references.insert(value.to_owned());
        }

        return VisitorResponse::StepOut;
    }
}

impl<'v> ReferencesCollector<'v> {
    fn check_unused(&self, errors: &mut CodegenErrors) {
        for (name, production) in &self.definitions.top_level_parsers {
            if self.definitions.required.contains(name) {
                continue;
            }

            if !self.references.contains(name) {
                errors.push(
                    &production.path,
                    production.cst_node.range(),
                    Errors::UnusedProduction(name.to_owned()),
                );
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("There is no Production '{0}'.")]
    NoSuchProduction(String),
    #[error("Cannot reference inner node '{0}', only top-level productions.")]
    NodesCannotBeReferenced(String),
    #[error("Scanner cannot reference Parser '{0}'.")]
    ScannersCanOnlyReferenceScanners(String),
    #[error("There is no Parser '{0}'.")]
    NoSuchPrimaryExpression(String),
    #[error("Production '{0}' is not used.")]
    UnusedProduction(String),
}
