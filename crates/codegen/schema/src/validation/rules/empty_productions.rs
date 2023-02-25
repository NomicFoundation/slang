use codegen_utils::errors::CodegenErrors;

use crate::validation::{
    ast::{
        productions::{ExpressionParser, ExpressionRef},
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
    fn visit_expression(
        &mut self,
        expression: &ExpressionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        if let Some(syntax) = find_empty_node(expression) {
            reporter.report(syntax, Errors::PossibleEmptyRoot);
        }

        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }
}

fn find_empty_node<'a>(expression: &'a ExpressionRef) -> Option<&'a crate::yaml::cst::NodeRef> {
    match &expression.parser.value {
        ExpressionParser::Choice(choices) => {
            return choices.iter().find_map(|choice| find_empty_node(choice));
        }

        ExpressionParser::Difference { minuend, .. } => {
            return find_empty_node(minuend);
        }

        ExpressionParser::OneOrMore(expression)
        | ExpressionParser::SeparatedBy { expression, .. } => {
            return find_empty_node(expression);
        }

        ExpressionParser::Optional(_) | ExpressionParser::ZeroOrMore(_) => {
            return Some(&expression.parser.syntax);
        }

        ExpressionParser::Sequence(sequence) => {
            if sequence.iter().all(|item| find_empty_node(item).is_some()) {
                return Some(&expression.parser.syntax);
            } else {
                return None;
            }
        }

        ExpressionParser::Repeat {
            min, expression, ..
        } => {
            if min.value > 0 {
                return find_empty_node(expression);
            } else {
                return Some(&min.syntax);
            }
        }

        ExpressionParser::DelimitedBy { .. }
        | ExpressionParser::Not { .. }
        | ExpressionParser::Range { .. }
        | ExpressionParser::Reference(_)
        | ExpressionParser::Terminal(_) => {
            return None; // Cannot be empty.
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("A production's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyRoot,
}
