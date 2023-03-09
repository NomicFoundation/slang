use codegen_utils::errors::CodegenErrors;

use crate::validation::{
    ast::{
        parser::{ParserDefinition, ParserRef},
        precedence_parser::PrecedenceParserRef,
        scanner::{ScannerDefinition, ScannerRef},
        visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
    },
    Model,
};

pub fn check(model: &Model, errors: &mut CodegenErrors) {
    let mut visitor = EmptyProductionsVisitor::new();

    visitor.visit(model, errors);
}

struct EmptyProductionsVisitor {}

impl EmptyProductionsVisitor {
    fn new() -> Self {
        return Self {};
    }
}

impl Visitor for EmptyProductionsVisitor {
    fn visit_scanner(&mut self, scanner: &ScannerRef, reporter: &mut Reporter) -> VisitorResponse {
        if let Some(cst_node) = find_empty_scanner_node(scanner) {
            reporter.report(cst_node, Errors::PossibleEmptyScanner);
        }

        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }

    fn visit_parser(&mut self, parser: &ParserRef, reporter: &mut Reporter) -> VisitorResponse {
        if let Some(cst_node) = find_empty_parser_node(parser) {
            reporter.report(cst_node, Errors::PossibleEmptyParser);
        }

        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }

    fn visit_precedence_parser(
        &mut self,
        _: &PrecedenceParserRef,
        _: &mut Reporter,
    ) -> VisitorResponse {
        // To ensure we don't visit the operator definitions, we return StepOut.
        return VisitorResponse::StepOut;
    }
}

fn find_empty_scanner_node<'a>(scanner: &'a ScannerRef) -> Option<&'a crate::yaml::cst::NodeRef> {
    match &scanner.definition.value {
        ScannerDefinition::Choice(choices) => {
            return choices
                .iter()
                .find_map(|choice| find_empty_scanner_node(choice));
        }

        ScannerDefinition::Difference { minuend, .. } => {
            return find_empty_scanner_node(minuend);
        }

        ScannerDefinition::OneOrMore(expression)
        | ScannerDefinition::TrailingContext { expression, .. } => {
            return find_empty_scanner_node(expression);
        }

        ScannerDefinition::Optional(_) | ScannerDefinition::ZeroOrMore(_) => {
            return Some(&scanner.definition.cst_node);
        }

        ScannerDefinition::Sequence(sequence) => {
            if sequence
                .iter()
                .all(|item| find_empty_scanner_node(item).is_some())
            {
                return Some(&scanner.definition.cst_node);
            } else {
                return None;
            }
        }

        ScannerDefinition::Repeat {
            min, expression, ..
        } => {
            if min.value > 0 {
                return find_empty_scanner_node(expression);
            } else {
                return Some(&min.cst_node);
            }
        }

        ScannerDefinition::Not { .. }
        | ScannerDefinition::Range { .. }
        | ScannerDefinition::Reference(_)
        | ScannerDefinition::Terminal(_) => {
            return None; // Cannot be empty.
        }
    };
}

fn find_empty_parser_node<'a>(parser: &'a ParserRef) -> Option<&'a crate::yaml::cst::NodeRef> {
    match &parser.definition.value {
        ParserDefinition::Choice(choices) => {
            return choices
                .iter()
                .find_map(|choice| find_empty_parser_node(choice));
        }

        ParserDefinition::OneOrMore(expression)
        | ParserDefinition::SeparatedBy { expression, .. }
        | ParserDefinition::TerminatedBy { expression, .. } => {
            return find_empty_parser_node(expression);
        }

        ParserDefinition::Optional(_) | ParserDefinition::ZeroOrMore(_) => {
            return Some(&parser.definition.cst_node);
        }

        ParserDefinition::Sequence(sequence) => {
            if sequence
                .iter()
                .all(|item| find_empty_parser_node(item).is_some())
            {
                return Some(&parser.definition.cst_node);
            } else {
                return None;
            }
        }

        ParserDefinition::Repeat {
            min, expression, ..
        } => {
            if min.value > 0 {
                return find_empty_parser_node(expression);
            } else {
                return Some(&min.cst_node);
            }
        }

        ParserDefinition::DelimitedBy { .. } | ParserDefinition::Reference(_) => {
            return None; // Cannot be empty.
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("A parser's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyParser,
    #[error("A scanner's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyScanner,
}
