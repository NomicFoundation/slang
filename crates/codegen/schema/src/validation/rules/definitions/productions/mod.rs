use std::collections::HashSet;

use crate::{
    types::{LanguageDefinitionRef, ProductionRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub struct Productions {
    language: LanguageDefinitionRef,
    productions_already_seen: HashSet<String>,
}

impl Productions {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let mut instance = Self {
            language: language.to_owned(),
            productions_already_seen: HashSet::new(),
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Productions {
    fn visit_manifest(&mut self, location: &LocationRef, reporter: &mut Reporter) -> bool {
        let required_productions = self.language.required_productions();

        for name in required_productions {
            if let Some(production) = self.language.productions.get(name) {
                if production.inlined {
                    reporter.report(location, Errors::RequiredCannotBeInlined(name.to_owned()));
                }
            } else {
                reporter.report(location, Errors::MissingRequired(name.to_owned()));
            }
        }

        true
    }

    fn visit_production(
        &mut self,
        production: &ProductionRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        let name = &production.name;
        if !self.productions_already_seen.insert(name.to_owned()) {
            reporter.report(location, Errors::DuplicateProduction(name.to_owned()));
        }

        false
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Required production '{0}' is not defined.")]
    MissingRequired(String),
    #[error("Required production '{0}' cannot be inlined.")]
    RequiredCannotBeInlined(String),
    #[error("Production '{0}' is defined more than once.")]
    DuplicateProduction(String),
}
