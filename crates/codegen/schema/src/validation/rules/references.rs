use std::collections::HashSet;

use codegen_utils::errors::CodegenErrors;

use crate::{
    validation::{
        ast::{
            node::Node,
            parser::ParserDefinition,
            precedence_parser::PrecedenceParserRef,
            scanner::ScannerDefinition,
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
    yaml::cst,
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
    fn visit_scanner_definition(
        &mut self,
        scanner: &ScannerDefinition,
        reporter: &mut Reporter,
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
        reporter: &mut Reporter,
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

    fn visit_precedence_parser(
        &mut self,
        parser: &PrecedenceParserRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        parser
            .definition
            .primary_expressions
            .iter()
            .for_each(|expression| {
                let Node { cst_node, value } = &expression.value.reference;
                self.check_is_scanner_or_parser_reference(cst_node, value, reporter);
            });
        return VisitorResponse::StepIn;
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
