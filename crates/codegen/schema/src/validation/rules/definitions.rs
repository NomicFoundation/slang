use std::collections::{HashMap, HashSet};

use codegen_utils::errors::CodegenErrors;

use crate::{
    validation::{
        ast::{
            files::{FilePathRef, TopicFile},
            productions::{ExpressionRef, ProductionRef},
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
            Node,
        },
        types::productions::ProductionKind,
        Model,
    },
    yaml::cst,
};

pub struct Definitions {
    pub required: HashSet<String>,
    pub productions: HashMap<String, Definition<ProductionRef>>,
    pub expressions: HashMap<String, Definition<ExpressionRef>>,
}

pub struct Definition<T> {
    pub path: FilePathRef,
    pub syntax: cst::NodeRef,
    pub value: T,
}

pub fn collect(model: &Model, errors: &mut CodegenErrors) -> Definitions {
    let mut collector = DefinitionCollector::new();

    collector.visit(model, errors);
    collector.check_required(model, errors);

    return collector.definitions;
}

struct DefinitionCollector {
    current_topic_path: Option<FilePathRef>,
    current_production_kind: Option<ProductionKind>,
    definitions: Definitions,
}

impl DefinitionCollector {
    fn new() -> Self {
        return Self {
            current_topic_path: None,
            current_production_kind: None,
            definitions: Definitions {
                required: HashSet::new(),
                productions: HashMap::new(),
                expressions: HashMap::new(),
            },
        };
    }
}

impl Visitor for DefinitionCollector {
    fn visit_topic(&mut self, topic: &TopicFile, _reporter: &mut Reporter) -> VisitorResponse {
        self.current_topic_path = Some(topic.path.to_owned());

        return VisitorResponse::StepIn;
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        self.current_production_kind = Some(production.kind.value);

        if self.check_uniqueness(&production.name.value, &production.name.syntax, reporter) {
            let definition = Definition {
                path: self.current_topic_path.as_ref().unwrap().to_owned(),
                syntax: production.name.syntax.to_owned(),
                value: production.to_owned(),
            };

            self.definitions
                .productions
                .insert(production.name.value.to_owned(), definition);
        }

        return VisitorResponse::StepIn;
    }

    fn visit_expression(
        &mut self,
        expression: &ExpressionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        let name = match &expression.config.name {
            Some(name) => name,
            None => return VisitorResponse::StepIn,
        };

        let current_production_kind = self.current_production_kind.unwrap();
        if current_production_kind != ProductionKind::Rule {
            reporter.report(&name.syntax, Errors::InvalidParent(current_production_kind));
            return VisitorResponse::StepIn;
        }

        if self.check_uniqueness(&name.value, &name.syntax, reporter) {
            let definition = Definition {
                path: self.current_topic_path.as_ref().unwrap().to_owned(),
                syntax: name.syntax.to_owned(),
                value: expression.to_owned(),
            };

            self.definitions
                .expressions
                .insert(name.value.to_owned(), definition);
        }

        return VisitorResponse::StepIn;
    }
}

impl DefinitionCollector {
    fn check_uniqueness(&self, name: &str, syntax: &cst::Node, reporter: &mut Reporter) -> bool {
        if self.definitions.productions.contains_key(name) {
            reporter.report(syntax, Errors::ExistingProduction(name.to_owned()));
            return false;
        }

        if self.definitions.expressions.contains_key(name) {
            reporter.report(syntax, Errors::ExistingExpression(name.to_owned()));
            return false;
        }

        return true;
    }

    fn check_required(&mut self, model: &Model, errors: &mut CodegenErrors) {
        let manifest = &model.manifest.ast.value;

        for name in ["LeadingTrivia", "TrailingTrivia"] {
            self.definitions.required.insert(name.to_owned());

            if !self.definitions.productions.contains_key(name) {
                errors.push(
                    &model.manifest.path,
                    manifest.title.syntax.range(),
                    Errors::Missing(name.to_owned()),
                );
            }
        }

        {
            let Node { syntax, value } = &manifest.root_production;
            self.definitions.required.insert(value.to_owned());

            if !self.definitions.productions.contains_key(value) {
                errors.push(
                    &model.manifest.path,
                    syntax.range(),
                    Errors::Missing(value.to_owned()),
                );
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not defined.")]
    Missing(String),
    #[error("Production '{0}' is already defined.")]
    ExistingProduction(String),
    #[error("Expression '{0}' is already defined.")]
    ExistingExpression(String),
    #[error("Sub-expressions of {0:?} cannot have explicit names.")]
    InvalidParent(ProductionKind),
}
