use std::path::PathBuf;

use crate::yaml::cst::NodeRef;

use super::yaml::files::File;

use super::types::{
    manifest::Manifest,
    parser::{ParserDefinition, ParserRef},
    precedence_parser::{OperatorDefinition, PrecedenceParserRef},
    production::{Production, ProductionRef, VersionMap},
    scanner::{ScannerDefinition, ScannerRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorResponse {
    StepIn,
    StepOut,
}

pub trait Visitor {
    fn visit_manifest_file(&mut self, _manifest_file: &File<Manifest>) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_manifest(
        &mut self,
        _manifest: &Manifest,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_productions_file(
        &mut self,
        _productions_file: &File<Vec<ProductionRef>>,
        manifest_path: &PathBuf,
        manifest_node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_production(
        &mut self,
        _production: &ProductionRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_scanner(
        &mut self,
        _scanner: &ScannerRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_scanner_definition(
        &mut self,
        _definition: &ScannerDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_parser(
        &mut self,
        _parser: &ParserRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_parser_definition(
        &mut self,
        _definition: &ParserDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_precedence_parser(
        &mut self,
        _precedence_parser: &PrecedenceParserRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_operator_definition(
        &mut self,
        _operator_definition: &OperatorDefinition,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }
}

impl File<Manifest> {
    fn visit<V: Visitor>(&self, visitor: &mut V) {
        if visitor.visit_manifest_file(self) == VisitorResponse::StepIn {
            self.value.visit(visitor, &self.path, &self.cst_node);
        }
    }
}

impl Manifest {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_manifest(self, path, node) == VisitorResponse::StepIn {
            for section in self.sections {
                for topic in section.topics {
                    for production in topic.productions {
                        production.visit(visitor, &topic.path, &topic.cst_node);
                    }
                }
            }
        }
    }
}

impl File<Vec<ProductionRef>> {
    fn visit<V: Visitor>(&self, visitor: &mut V, manifest_path: &PathBuf, manifest_node: &NodeRef) {
        if visitor.visit_productions_file(self, manifest_path, manifest_node)
            == VisitorResponse::StepIn
        {
            for production in &self.value {
                production.visit(visitor);
            }
        }
    }
}

impl ProductionRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_production(self, path, node) == VisitorResponse::StepIn {
            match self.as_ref() {
                Production::Scanner {
                    version_map: versioning,
                    ..
                } => match versioning {
                    VersionMap::Unversioned(scanner) => {
                        scanner.visit(visitor);
                    }
                    VersionMap::Versioned(versions) => {
                        for scanner in versions.values() {
                            scanner.visit(visitor)
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
                } => match versioning {
                    VersionMap::Unversioned(parser) => {
                        parser.visit(visitor);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            parser.visit(visitor)
                        }
                    }
                },

                Production::PrecedenceParser {
                    version_map: versioning,
                    ..
                } => match versioning {
                    VersionMap::Unversioned(parser) => {
                        parser.visit(visitor);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            parser.visit(visitor)
                        }
                    }
                },
            }
        }
    }
}

impl ScannerRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_scanner(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor);
        }
    }
}

impl ScannerDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_scanner_definition(self, path, node) == VisitorResponse::StepIn {
            match &self {
                ScannerDefinition::Choice(expressions)
                | ScannerDefinition::Sequence(expressions) => {
                    for expression in expressions {
                        expression.visit(visitor);
                    }
                }
                ScannerDefinition::Repeat { expression, .. }
                | ScannerDefinition::Not(expression)
                | ScannerDefinition::OneOrMore(expression)
                | ScannerDefinition::Optional(expression)
                | ScannerDefinition::ZeroOrMore(expression) => {
                    expression.visit(visitor);
                }
                ScannerDefinition::TrailingContext {
                    expression,
                    not_followed_by,
                } => {
                    expression.visit(visitor);
                    not_followed_by.visit(visitor);
                }
                ScannerDefinition::Difference {
                    minuend,
                    subtrahend,
                } => {
                    minuend.visit(visitor);
                    subtrahend.visit(visitor);
                }
                ScannerDefinition::Range { .. }
                | ScannerDefinition::Reference(_)
                | ScannerDefinition::Terminal(_) => {}
            };
        }
    }
}

impl ParserRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_parser(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor);
        }
    }
}

impl ParserDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_parser_definition(self, path, node) == VisitorResponse::StepIn {
            match &self {
                ParserDefinition::Choice(expressions) | ParserDefinition::Sequence(expressions) => {
                    for expression in expressions {
                        expression.visit(visitor);
                    }
                }
                ParserDefinition::DelimitedBy { expression, .. }
                | ParserDefinition::SeparatedBy { expression, .. }
                | ParserDefinition::TerminatedBy { expression, .. }
                | ParserDefinition::Repeat { expression, .. }
                | ParserDefinition::OneOrMore(expression)
                | ParserDefinition::Optional(expression)
                | ParserDefinition::ZeroOrMore(expression) => {
                    expression.visit(visitor);
                }
                ParserDefinition::Reference(_) => {}
            };
        }
    }
}

impl PrecedenceParserRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_precedence_parser(self, path, node) == VisitorResponse::StepIn {
            for operator in &self.definition.operators {
                operator.visit(visitor);
            }
        }
    }
}

impl OperatorDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_operator_definition(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor)
        }
    }
}
