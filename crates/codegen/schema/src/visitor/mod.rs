use std::path::PathBuf;

use crate::{validation::model::Model, yaml::cst::NodeRef};

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
    fn visit_manifest(
        &mut self,
        manifest: &Manifest,
        path: &PathBuf,
        node: &NodeRef,
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

impl Model {
    pub fn visit<V: Visitor>(&self, visitor: &mut V) {}
}

pub trait Visitable {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef);
}

impl Visitable for ProductionRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_production(self, path, node) == VisitorResponse::StepIn {
            match self.as_ref() {
                Production::Scanner {
                    version_map: versioning,
                    ..
                } => match versioning {
                    VersionMap::Unversioned(scanner) => {
                        scanner.visit(visitor, path, node);
                    }
                    VersionMap::Versioned(versions) => {
                        for scanner in versions.values() {
                            if let Some(scanner) = scanner {
                                scanner.visit(visitor, path, node)
                            }
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
                        parser.visit(visitor, path, node);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            if let Some(parser) = parser {
                                parser.visit(visitor, path, node)
                            }
                        }
                    }
                },

                Production::PrecedenceParser {
                    version_map: versioning,
                    ..
                } => match versioning {
                    VersionMap::Unversioned(parser) => {
                        parser.visit(visitor, path, node);
                    }
                    VersionMap::Versioned(versions) => {
                        for parser in versions.values() {
                            if let Some(parser) = parser {
                                parser.visit(visitor, path, node)
                            }
                        }
                    }
                },
            }
        }
    }
}

impl Visitable for ScannerRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_scanner(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor, path, node);
        }
    }
}

impl Visitable for ScannerDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_scanner_definition(self, path, node) == VisitorResponse::StepIn {
            match &self {
                ScannerDefinition::Choice(expressions)
                | ScannerDefinition::Sequence(expressions) => {
                    for expression in expressions {
                        expression.visit(visitor, path, node);
                    }
                }
                ScannerDefinition::Repeat { expression, .. }
                | ScannerDefinition::Not(expression)
                | ScannerDefinition::OneOrMore(expression)
                | ScannerDefinition::Optional(expression)
                | ScannerDefinition::ZeroOrMore(expression) => {
                    expression.visit(visitor, path, node);
                }
                ScannerDefinition::TrailingContext {
                    expression,
                    not_followed_by,
                } => {
                    expression.visit(visitor, path, node);
                    not_followed_by.visit(visitor, path, node);
                }
                ScannerDefinition::Difference {
                    minuend,
                    subtrahend,
                } => {
                    minuend.visit(visitor, path, node);
                    subtrahend.visit(visitor, path, node);
                }
                ScannerDefinition::Range { .. }
                | ScannerDefinition::Reference(_)
                | ScannerDefinition::Terminal(_) => {}
            };
        }
    }
}

impl Visitable for ParserRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_parser(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor, path, node);
        }
    }
}

impl Visitable for ParserDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_parser_definition(self, path, node) == VisitorResponse::StepIn {
            match &self {
                ParserDefinition::Choice(expressions) | ParserDefinition::Sequence(expressions) => {
                    for expression in expressions {
                        expression.visit(visitor, path, node);
                    }
                }
                ParserDefinition::DelimitedBy { expression, .. }
                | ParserDefinition::SeparatedBy { expression, .. }
                | ParserDefinition::TerminatedBy { expression, .. }
                | ParserDefinition::Repeat { expression, .. }
                | ParserDefinition::OneOrMore(expression)
                | ParserDefinition::Optional(expression)
                | ParserDefinition::ZeroOrMore(expression) => {
                    expression.visit(visitor, path, node);
                }
                ParserDefinition::Reference(_) => {}
            };
        }
    }
}

impl Visitable for PrecedenceParserRef {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_precedence_parser(self, path, node) == VisitorResponse::StepIn {
            for operator in &self.definition.operators {
                operator.visit(visitor, path, node);
            }
            self.definition
                .primary_expression
                .visit(visitor, path, node);
        }
    }
}

impl Visitable for OperatorDefinition {
    fn visit<V: Visitor>(&self, visitor: &mut V, path: &PathBuf, node: &NodeRef) {
        if visitor.visit_operator_definition(self, path, node) == VisitorResponse::StepIn {
            self.definition.visit(visitor, path, node)
        }
    }
}
