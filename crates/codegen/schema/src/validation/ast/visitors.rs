use std::rc::Rc;

use codegen_utils::errors::CodegenErrors;

use crate::{validation::Model, yaml::cst};

use super::{
    files::{ManifestFile, PathBufRef, TopicFile},
    parser::{Parser, ParserDefinition, ParserRef},
    precedence_parser::{OperatorDefinition, PrecedenceParser, PrecedenceParserRef, Reference},
    production::{Production, ProductionRef, VersionMap},
    scanner::{Scanner, ScannerDefinition, ScannerRef},
};

pub struct Reporter<'v> {
    path: &'v PathBufRef,
    errors: &'v mut CodegenErrors,
}

impl<'v> Reporter<'v> {
    fn new(path: &'v PathBufRef, errors: &'v mut CodegenErrors) -> Self {
        Self { path, errors }
    }

    pub fn report<E: std::error::Error>(&mut self, cst_node: &cst::Node, error: E) {
        self.errors.push(self.path, cst_node.range(), error);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorResponse {
    StepIn,
    StepOut,
}

pub trait Visitor {
    fn visit_manifest(
        &mut self,
        _manifest_file: &ManifestFile,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_topic(
        &mut self,
        _topic_file: &TopicFile,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_production(
        &mut self,
        _production: &ProductionRef,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_scanner(
        &mut self,
        _scanner: &Rc<Scanner>,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_parser(&mut self, _parser: &Rc<Parser>, _reporter: &mut Reporter) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_precedence_parser(
        &mut self,
        _precedence_parser: &Rc<PrecedenceParser>,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_operator_definition(
        &mut self,
        _operator_definition: &OperatorDefinition,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_primary_expression_reference(
        &mut self,
        _reference: &Reference,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }
}

pub trait VisitorExtensions {
    fn visit(&mut self, model: &Model, errors: &mut CodegenErrors);
}

impl<V: Visitor> VisitorExtensions for V {
    fn visit(&mut self, model: &Model, errors: &mut CodegenErrors) {
        {
            let mut reporter = Reporter::new(&model.manifest_file.path, errors);
            model.manifest_file.receive(self, &mut reporter);
        }

        for topic in &model.topic_files {
            let mut reporter = Reporter::new(&topic.path, errors);
            topic.receive(self, &mut reporter);
        }
    }
}

trait Receiver {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter);
}

impl Receiver for ManifestFile {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        visitor.visit_manifest(self, reporter);
    }
}

impl Receiver for TopicFile {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_topic(self, reporter) == VisitorResponse::StepIn {
            for production in &self.ast.value {
                production.receive(visitor, reporter);
            }
        }
    }
}

impl Receiver for ProductionRef {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_production(self, reporter) == VisitorResponse::StepIn {
            match &**self {
                Production::Scanner {
                    version_map: versioning,
                    ..
                } => match &versioning.value {
                    VersionMap::Unversioned(scanner) => {
                        scanner.receive(visitor, reporter);
                    }
                    VersionMap::Versioned(versions) => {
                        for scanner in versions.values() {
                            scanner.receive(visitor, reporter);
                        }
                    }
                },

                Production::TriviaParser {
                    version_map: versioning,
                    ..
                }
                | Production::Parser {
                    version_map: versioning,
                    ..
                } => match &versioning.value {
                    VersionMap::Unversioned(parser) => {
                        parser.receive(visitor, reporter);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            parser.receive(visitor, reporter);
                        }
                    }
                },

                Production::PrecedenceParser {
                    version_map: versioning,
                    ..
                } => match &versioning.value {
                    VersionMap::Unversioned(parser) => {
                        parser.receive(visitor, reporter);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            parser.receive(visitor, reporter);
                        }
                    }
                },
            }
        }
    }
}

impl Receiver for ScannerRef {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_scanner(self, reporter) == VisitorResponse::StepIn {
            self.definition.value.receive(visitor, reporter);
        }
    }
}

impl Receiver for ScannerDefinition {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        match &self {
            ScannerDefinition::Choice(expressions) | ScannerDefinition::Sequence(expressions) => {
                for expression in expressions {
                    expression.receive(visitor, reporter);
                }
            }
            ScannerDefinition::DelimitedBy { expression, .. }
            | ScannerDefinition::Repeat { expression, .. }
            | ScannerDefinition::SeparatedBy { expression, .. }
            | ScannerDefinition::Not(expression)
            | ScannerDefinition::OneOrMore(expression)
            | ScannerDefinition::Optional(expression)
            | ScannerDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, reporter);
            }
            ScannerDefinition::TrailingContext {
                expression,
                not_followed_by,
            } => {
                expression.receive(visitor, reporter);
                not_followed_by.receive(visitor, reporter);
            }
            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                minuend.receive(visitor, reporter);
                subtrahend.receive(visitor, reporter);
            }
            ScannerDefinition::Range { .. }
            | ScannerDefinition::Reference(_)
            | ScannerDefinition::Terminal(_) => {}
        };
    }
}

impl Receiver for ParserRef {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_parser(self, reporter) == VisitorResponse::StepIn {
            self.definition.value.receive(visitor, reporter);
        }
    }
}

impl Receiver for ParserDefinition {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        match &self {
            ParserDefinition::Choice(expressions) | ParserDefinition::Sequence(expressions) => {
                for expression in expressions {
                    expression.receive(visitor, reporter);
                }
            }
            ParserDefinition::DelimitedBy { expression, .. }
            | ParserDefinition::Repeat { expression, .. }
            | ParserDefinition::SeparatedBy { expression, .. }
            | ParserDefinition::OneOrMore(expression)
            | ParserDefinition::Optional(expression)
            | ParserDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, reporter);
            }
            ParserDefinition::Reference(_) | ParserDefinition::Terminal(_) => {}
        };
    }
}

impl Receiver for PrecedenceParserRef {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_precedence_parser(self, reporter) == VisitorResponse::StepIn {
            for operator in &self.definition.operators {
                operator.value.receive(visitor, reporter);
            }
            for primary_expression in &self.definition.primary_expressions {
                primary_expression.value.receive(visitor, reporter);
            }
        }
    }
}

impl Receiver for OperatorDefinition {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        if visitor.visit_operator_definition(self, reporter) == VisitorResponse::StepIn {
            self.definition.value.receive(visitor, reporter)
        }
    }
}

impl Receiver for Reference {
    fn receive<V: Visitor>(&self, visitor: &mut V, reporter: &mut Reporter) {
        visitor.visit_primary_expression_reference(self, reporter);
    }
}
