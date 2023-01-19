use codegen_utils::errors::CodegenErrors;

use crate::{
    types::productions::ProductionKind,
    validation::{
        ast::{
            productions::{EBNFRepeat, ExpressionRef, ProductionRef, ProductionVersioning, EBNF},
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
                // Cannot be empty
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
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Token's root expression cannot be possibly empty. Specify the arity on the expression(s) that reference it.")]
    PossibleEmptyRoot,
}
