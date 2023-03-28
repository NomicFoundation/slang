use std::path::PathBuf;

use codegen_utils::errors::CodegenErrors;

use crate::{
    types::parser::ParserDefinition,
    types::scanner::ScannerDefinition,
    visitor::{Visitor, VisitorResponse},
    yaml::cst::NodeRef,
};

pub struct Validator<'ce> {
    errors: &'ce mut CodegenErrors,
}

impl<'ce> Validator<'ce> {
    pub fn new(errors: &'ce mut CodegenErrors) -> Self {
        return Self { errors };
    }
}

impl Visitor for Validator<'_> {
    fn visit_scanner_definition(
        &mut self,
        definition: &ScannerDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        if definition.produces_epsilon() {
            self.errors
                .push(path, node.range(), Errors::PossibleEmptyScanner);
        }
        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }

    fn visit_parser_definition(
        &mut self,
        definition: &ParserDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        if definition.produces_epsilon() {
            self.errors
                .push(path, node.range(), Errors::PossibleEmptyParser);
        }
        // Only check the top-most expression. Ignore nested ones.
        return VisitorResponse::StepOut;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("A parser's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyParser,
    #[error("A scanner's root expression cannot be optionally empty. Refactor usages to specify the arity instead.")]
    PossibleEmptyScanner,
}
