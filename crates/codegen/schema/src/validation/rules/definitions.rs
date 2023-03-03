use std::collections::{HashMap, HashSet};

use codegen_utils::errors::CodegenErrors;

use crate::{
    validation::{
        ast::{
            files::{PathBufRef, ProductionsFile},
            node::Node,
            parser::ParserRef,
            production::{Production, ProductionRef},
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
    yaml::cst,
};

pub struct Definitions {
    pub required: HashSet<String>,
    pub top_level_scanners: HashMap<String, Definition<ProductionRef>>,
    pub top_level_parsers: HashMap<String, Definition<ProductionRef>>,
    pub internal_named_parsers: HashMap<String, Definition<ParserRef>>,
}

pub struct Definition<T> {
    pub path: PathBufRef,
    pub cst_node: cst::NodeRef,
    pub value: T,
}

pub fn collect(model: &Model, errors: &mut CodegenErrors) -> Definitions {
    let mut collector = DefinitionCollector::new();

    collector.visit(model, errors);
    collector.check_required(model, errors);

    return collector.definitions;
}

struct DefinitionCollector {
    current_file_path: Option<PathBufRef>,
    definitions: Definitions,
}

impl DefinitionCollector {
    fn new() -> Self {
        return Self {
            current_file_path: None,
            definitions: Definitions {
                required: HashSet::new(),
                top_level_scanners: HashMap::new(),
                top_level_parsers: HashMap::new(),
                internal_named_parsers: HashMap::new(),
            },
        };
    }
}

impl Visitor for DefinitionCollector {
    fn visit_productions_file(
        &mut self,
        productions_file: &ProductionsFile,
        _reporter: &mut Reporter,
    ) -> VisitorResponse {
        self.current_file_path = Some(productions_file.path.to_owned());

        return VisitorResponse::StepIn;
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        if self.check_uniqueness(production.name_node(), reporter) {
            let definition = Definition {
                path: self.current_file_path.as_ref().unwrap().to_owned(),
                cst_node: production.name_node().cst_node.to_owned(),
                value: production.to_owned(),
            };

            match production.as_ref() {
                Production::Scanner { name, .. } => self
                    .definitions
                    .top_level_scanners
                    .insert(name.value.to_owned(), definition),
                Production::TriviaParser { name, .. }
                | Production::Parser { name, .. }
                | Production::PrecedenceParser { name, .. } => self
                    .definitions
                    .top_level_parsers
                    .insert(name.value.to_owned(), definition),
            };
        }

        return VisitorResponse::StepIn;
    }

    fn visit_parser(&mut self, parser: &ParserRef, reporter: &mut Reporter) -> VisitorResponse {
        let name = match &parser.name {
            Some(name) => name,
            None => return VisitorResponse::StepIn,
        };

        if self.check_uniqueness(&name, reporter) {
            let definition = Definition {
                path: self.current_file_path.as_ref().unwrap().to_owned(),
                cst_node: name.cst_node.to_owned(),
                value: parser.to_owned(),
            };

            self.definitions
                .internal_named_parsers
                .insert(name.value.to_owned(), definition);
        }

        return VisitorResponse::StepIn;
    }
}

impl DefinitionCollector {
    fn check_uniqueness(&self, name: &Node<String>, reporter: &mut Reporter) -> bool {
        if self.definitions.top_level_parsers.contains_key(&name.value) {
            reporter.report(
                &name.cst_node,
                Errors::DuplicateProduction(name.value.to_owned()),
            );
            return false;
        }

        if self
            .definitions
            .top_level_scanners
            .contains_key(&name.value)
        {
            reporter.report(
                &name.cst_node,
                Errors::DuplicateProduction(name.value.to_owned()),
            );
            return false;
        }

        if self
            .definitions
            .internal_named_parsers
            .contains_key(&name.value)
        {
            reporter.report(
                &name.cst_node,
                Errors::DuplicateProduction(name.value.to_owned()),
            );
            return false;
        }

        return true;
    }

    fn check_required(&mut self, model: &Model, errors: &mut CodegenErrors) {
        let manifest = &model.manifest_file.ast.value;

        for name in ["LeadingTrivia", "TrailingTrivia"] {
            self.definitions.required.insert(name.to_owned());

            if !self.definitions.top_level_parsers.contains_key(name) {
                errors.push(
                    &model.manifest_file.path,
                    manifest.title.cst_node.range(),
                    Errors::MissingRequiredParser(name.to_owned()),
                );
            }
        }

        {
            let Node { cst_node, value } = &manifest.root_production;
            self.definitions.required.insert(value.to_owned());

            if !self.definitions.top_level_parsers.contains_key(value) {
                errors.push(
                    &model.manifest_file.path,
                    cst_node.range(),
                    Errors::MissingRootProduction(value.to_owned()),
                );
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Duplicate Definition '{0}'.")]
    DuplicateProduction(String),
    #[error("Required Parser '{0}' is not defined.")]
    MissingRequiredParser(String),
    #[error("Root Production '{0}' is not defined.")]
    MissingRootProduction(String),
}
