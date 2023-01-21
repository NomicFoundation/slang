use std::collections::HashSet;

use codegen_utils::errors::CodegenErrors;

use crate::validation::{
    ast::{
        productions::{ExpressionRef, EBNF},
        visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        Node,
    },
    rules::definitions::Definitions,
    Model,
};

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
    fn visit_expression(
        &mut self,
        expression: &ExpressionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        if let EBNF::Reference(reference) = &expression.ebnf.value {
            let Node { syntax, value } = reference;
            if self.definitions.expressions.contains_key(value) {
                reporter.report(syntax, Errors::Illegal(value.to_owned()))
            } else if !self.definitions.productions.contains_key(value) {
                reporter.report(syntax, Errors::NotFound(value.to_owned()))
            } else {
                self.references.insert(reference.value.to_owned());
            }

            return VisitorResponse::StepOut;
        } else {
            return VisitorResponse::StepIn;
        }
    }
}

impl<'v> ReferencesCollector<'v> {
    fn check_unused(&self, errors: &mut CodegenErrors) {
        for (name, production) in &self.definitions.productions {
            if self.definitions.required.contains(name) {
                continue;
            }

            if !self.references.contains(name) {
                errors.push(
                    &production.path,
                    production.syntax.range(),
                    Errors::Unused(name.to_owned()),
                );
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not defined.")]
    NotFound(String),
    #[error("Expression '{0}' cannot be used as a reference. References are only valid to top-level productions.")]
    Illegal(String),
    #[error("Production '{0}' is not used.")]
    Unused(String),
}
