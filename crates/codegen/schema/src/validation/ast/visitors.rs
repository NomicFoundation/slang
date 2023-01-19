use codegen_utils::errors::CodegenErrors;

use crate::{
    validation::{
        ast::{
            files::{FilePathRef, ManifestFile, TopicFile},
            productions::{
                EBNFDelimitedBy, EBNFRepeat, EBNFSeparatedBy, ExpressionRef, ProductionRef,
                ProductionVersioning, EBNF,
            },
        },
        Model,
    },
    yaml::cst,
};

pub struct Reporter<'v> {
    path: &'v FilePathRef,
    errors: &'v mut CodegenErrors,
}

impl<'v> Reporter<'v> {
    fn new(path: &'v FilePathRef, errors: &'v mut CodegenErrors) -> Self {
        Self { path, errors }
    }

    pub fn report<TError: std::error::Error>(&mut self, syntax: &cst::Node, error: TError) {
        self.errors.push(self.path, syntax.range(), error);
    }
}

pub enum VisitorResponse {
    StepIn,
    StepOut,
}

pub trait Visitor {
    fn visit_manifest(
        &mut self,
        _manifest: &ManifestFile,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_topic(&mut self, _topic: &TopicFile, _reporter: &mut Reporter) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_production(
        &mut self,
        _production: &ProductionRef,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }

    fn visit_expression(
        &mut self,
        _expression: &ExpressionRef,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        return VisitorResponse::StepIn;
    }
}

pub trait VisitorExtensions {
    fn visit(&mut self, model: &Model, errors: &mut CodegenErrors);
}

impl<TVisitor: Visitor> VisitorExtensions for TVisitor {
    fn visit(&mut self, model: &Model, errors: &mut CodegenErrors) {
        {
            let mut reporter = Reporter::new(&model.manifest.path, errors);
            model.manifest.receive(self, &mut reporter);
        }

        for topic in &model.topics {
            let mut reporter = Reporter::new(&topic.path, errors);
            topic.receive(self, &mut reporter);
        }
    }
}

trait Receiver {
    fn receive<TVisitor: Visitor>(&self, visitor: &mut TVisitor, reporter: &mut Reporter);
}

impl Receiver for ManifestFile {
    fn receive<TVisitor: Visitor>(&self, visitor: &mut TVisitor, reporter: &mut Reporter) {
        match visitor.visit_manifest(self, reporter) {
            VisitorResponse::StepOut => return,
            VisitorResponse::StepIn => { /* Continue */ }
        };

        // No-op for now. Can add manifest-specific extensions later.
    }
}

impl Receiver for TopicFile {
    fn receive<TVisitor: Visitor>(&self, visitor: &mut TVisitor, reporter: &mut Reporter) {
        match visitor.visit_topic(self, reporter) {
            VisitorResponse::StepOut => return,
            VisitorResponse::StepIn => { /* Continue */ }
        };

        for production in &self.ast.value {
            production.receive(visitor, reporter);
        }
    }
}

impl Receiver for ProductionRef {
    fn receive<TVisitor: Visitor>(&self, visitor: &mut TVisitor, reporter: &mut Reporter) {
        match visitor.visit_production(self, reporter) {
            VisitorResponse::StepOut => return,
            VisitorResponse::StepIn => { /* Continue */ }
        };

        match &self.versioning.value {
            ProductionVersioning::Unversioned(expression) => {
                expression.receive(visitor, reporter);
            }
            ProductionVersioning::Versioned(versions) => {
                for expression in versions.values() {
                    expression.receive(visitor, reporter);
                }
            }
        };
    }
}

impl Receiver for ExpressionRef {
    fn receive<TVisitor: Visitor>(&self, visitor: &mut TVisitor, reporter: &mut Reporter) {
        match visitor.visit_expression(self, reporter) {
            VisitorResponse::StepOut => return,
            VisitorResponse::StepIn => { /* Continue */ }
        };

        match &self.ebnf.value {
            EBNF::Choice(expressions) | EBNF::Sequence(expressions) => {
                for expression in expressions {
                    expression.receive(visitor, reporter);
                }
            }
            EBNF::DelimitedBy(EBNFDelimitedBy { expression, .. })
            | EBNF::Repeat(EBNFRepeat { expression, .. })
            | EBNF::SeparatedBy(EBNFSeparatedBy { expression, .. })
            | EBNF::Not(expression)
            | EBNF::OneOrMore(expression)
            | EBNF::Optional(expression)
            | EBNF::ZeroOrMore(expression) => {
                expression.receive(visitor, reporter);
            }
            EBNF::Difference(expression) => {
                expression.minuend.receive(visitor, reporter);
                expression.subtrahend.receive(visitor, reporter);
            }
            EBNF::Range(_) | EBNF::Reference(_) | EBNF::Terminal(_) => {}
        };
    }
}
