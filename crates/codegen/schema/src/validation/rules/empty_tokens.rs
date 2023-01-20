use codegen_utils::errors::CodegenErrors;

use crate::{
    types::productions::ProductionKind,
    validation::{
        ast::{
            productions::{ExpressionParser, ExpressionRef, ProductionRef, ProductionVersioning},
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
};

pub fn check(model: &Model, errors: &mut CodegenErrors) {
    let mut visitor = EmptyTokensVisitor::new();

    visitor.visit(model, errors);
}

struct EmptyTokensVisitor {}

impl EmptyTokensVisitor {
    fn new() -> Self {
        return Self {};
    }
}

impl Visitor for EmptyTokensVisitor {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        match production.kind.value {
            ProductionKind::Rule | ProductionKind::Trivia => {
                // not a token
            }
            ProductionKind::Token => {
                match &production.versioning.value {
                    ProductionVersioning::Unversioned(expression) => {
                        self.check_empty(expression, reporter);
                    }
                    ProductionVersioning::Versioned(versions) => {
                        for expression in versions.values() {
                            self.check_empty(expression, reporter);
                        }
                    }
                };
            }
        };

        return VisitorResponse::StepOut;
    }
}

impl EmptyTokensVisitor {
    fn check_empty(&mut self, expression: &ExpressionRef, reporter: &mut Reporter) {
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
                // Cannot be empty
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
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Token's root expression cannot be possibly empty. Specify the arity on the expression(s) that reference it.")]
    PossibleEmptyRoot,
}
