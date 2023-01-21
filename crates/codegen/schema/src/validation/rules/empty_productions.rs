use codegen_utils::errors::CodegenErrors;

use crate::validation::{
    ast::{
        productions::{EBNFRepeat, ExpressionRef, ProductionRef, EBNF},
        visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
    },
    types::productions::ProductionKind,
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
        match &expression.ebnf.value {
            EBNF::Choice(_)
            | EBNF::DelimitedBy(_)
            | EBNF::Difference(_)
            | EBNF::Not(_)
            | EBNF::OneOrMore(_)
            | EBNF::Range(_)
            | EBNF::Reference(_)
            | EBNF::SeparatedBy(_)
            | EBNF::Sequence(_)
            | EBNF::Terminal(_) => {
                // Cannot be optionally empty. Do nothing.
            }
            EBNF::Optional(_) | EBNF::ZeroOrMore(_) => {
                reporter.report(&expression.ebnf.syntax, Errors::PossibleEmptyRoot);
            }
            EBNF::Repeat(EBNFRepeat { min, .. }) => {
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
