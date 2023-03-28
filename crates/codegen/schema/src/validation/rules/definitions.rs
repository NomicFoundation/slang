use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use codegen_utils::errors::CodegenErrors;

use crate::{
    types::parser::ParserRef,
    types::production::Production,
    types::production::ProductionRef,
    visitor::{Visitor, VisitorResponse},
    yaml::cst::NodeRef,
};

pub struct Definition<T> {
    pub value: T,
    pub path: PathBuf,
    pub cst_node: NodeRef,
}

pub struct Validator<'ce> {
    errors: &'ce mut CodegenErrors,
    pub required: HashSet<String>,
    pub top_level_scanners: HashMap<String, Definition<ProductionRef>>,
    pub top_level_parsers: HashMap<String, Definition<ProductionRef>>,
    pub internal_named_parsers: HashMap<String, Definition<ParserRef>>,
}

impl<'ce> Validator<'ce> {
    pub fn new(errors: &'ce mut CodegenErrors) -> Self {
        return Self {
            errors,
            required: HashSet::new(),
            top_level_scanners: HashMap::new(),
            top_level_parsers: HashMap::new(),
            internal_named_parsers: HashMap::new(),
        };
    }
}

impl Visitor for Validator<'_> {
    // fn exit_manifest(&mut self, manifest: &Manifest
    //     path: &PathBuf,
    //     node: &NodeRef) {

    //     for name in ["LeadingTrivia", "TrailingTrivia"] {
    //         self.definitions.required.insert(name.to_owned());

    //         if !self.definitions.top_level_parsers.contains_key(name) {
    //             errors.push(
    //                 &model.manifest_file.path,
    //                 manifest.title.cst_node.range(),
    //                 Errors::MissingRequiredParser(name.to_owned()),
    //             );
    //         }
    //     }

    //     {
    //         let Node { cst_node, value } = &manifest.root_production;
    //         self.definitions.required.insert(value.to_owned());

    //         if !self.definitions.top_level_parsers.contains_key(value) {
    //             errors.push(
    //                 &model.manifest_file.path,
    //                 cst_node.range(),
    //                 Errors::MissingRootProduction(value.to_owned()),
    //             );
    //         }
    //     }
    // }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        if self.check_uniqueness(production.name(), path, &node.value_of_field("name")) {
            let definition = Definition {
                value: production.clone(),
                path: path.clone(),
                cst_node: node.clone(),
            };

            match production.as_ref() {
                Production::Scanner { name, .. } => {
                    self.top_level_scanners.insert(name.clone(), definition)
                }
                Production::TriviaParser { name, .. }
                | Production::Parser { name, .. }
                | Production::PrecedenceParser { name, .. } => {
                    self.top_level_parsers.insert(name.clone(), definition)
                }
            };
        }

        return VisitorResponse::StepIn;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        path: &PathBuf,
        node: &NodeRef,
    ) -> VisitorResponse {
        let name = match &parser.name {
            Some(name) => name,
            None => return VisitorResponse::StepIn,
        };

        if self.check_uniqueness(name, path, &node.value_of_field("name")) {
            let definition = Definition {
                value: parser.clone(),
                path: path.clone(),
                cst_node: node.clone(),
            };

            self.internal_named_parsers.insert(name.clone(), definition);
        }

        return VisitorResponse::StepIn;
    }
}

impl Validator<'_> {
    fn check_uniqueness(&self, name: &String, path: &PathBuf, node: &NodeRef) -> bool {
        if self.top_level_parsers.contains_key(name)
            || self.top_level_scanners.contains_key(name)
            || self.internal_named_parsers.contains_key(name)
        {
            self.errors.push(
                path,
                node.range(),
                Errors::DuplicateProduction(name.clone()),
            );
            return false;
        }

        return true;
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
