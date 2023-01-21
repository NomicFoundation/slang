use codegen_utils::errors::CodegenErrors;

use crate::{
    types::productions::ProductionKind,
    validation::{
        ast::{
            productions::{ExpressionParser, ExpressionRef, ProductionRef},
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
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
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        match production.kind.value {
            ProductionKind::Trivia => {
                // Skip. Allowed to be empty.
                return VisitorResponse::StepOut;
            }
            ProductionKind::Token | ProductionKind::Rule => {
                // Check the root expression.
                return VisitorResponse::StepIn;
            }
        };
    }

    fn visit_expression(
        &mut self,
        expression: &ExpressionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        match &expression.parser.value {
            ExpressionParser::Choice(_)
            | ExpressionParser::DelimitedBy { .. }
            | ExpressionParser::Difference { .. }
            | ExpressionParser::Not { .. }
            | ExpressionParser::OneOrMore(_)
            | ExpressionParser::Range { .. }
            | ExpressionParser::Reference(_)
            | ExpressionParser::SeparatedBy { .. }
            | ExpressionParser::Sequence(_)
            | ExpressionParser::Terminal(_) => {
                // Cannot be optionally empty. Do nothing.
            }
            ExpressionParser::Optional(_) | ExpressionParser::ZeroOrMore(_) => {
                reporter.report(&expression.parser.syntax, Errors::PossibleEmptyRoot);
            }
            ExpressionParser::Repeat { min, .. } => {
                if min.value == 0 {
                    reporter.report(&min.syntax, Errors::PossibleEmptyRoot);
                }
            }
        };

        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("A production's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyRoot,
}
