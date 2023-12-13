use std::collections::HashMap;

use crate::types::{LanguageDefinitionRef, PrecedenceParserRef, ProductionRef};
use crate::validation::visitors::{run_visitor, LocationRef, Reporter, Visitor};

pub struct Operators {
    language: LanguageDefinitionRef,
    current_production: Option<ProductionRef>,
    operators_already_seen: HashMap<String, ProductionRef>,
}

impl Operators {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let mut instance = Self {
            language: language.to_owned(),

            current_production: None,
            operators_already_seen: HashMap::new(),
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Operators {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_production = Some(production.to_owned());
        true
    }

    fn visit_parser(
        &mut self,
        _parser: &crate::types::ParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        false // skip
    }

    fn visit_scanner(
        &mut self,
        _scanner: &crate::types::ScannerRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        false // skip
    }

    fn visit_precedence_parser(
        &mut self,
        parser: &PrecedenceParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        for expression in &parser.operator_expressions {
            let name = &expression.name;
            if self.language.productions.contains_key(name) {
                reporter.report(location, Errors::OperatorNamedAsProduction(name.to_owned()));
                continue;
            }

            let current_production = self.current_production.as_ref().unwrap();

            match self.operators_already_seen.get(name) {
                Some(existing_production) => {
                    if current_production.name == existing_production.name {
                        // Operators can share a common name under the same production.
                        continue;
                    }

                    reporter.report(
                        location,
                        Errors::OperatorDefinedInAnotherProduction(
                            name.to_owned(),
                            existing_production.name.to_owned(),
                        ),
                    );
                }
                None => {
                    self.operators_already_seen
                        .insert(name.to_owned(), current_production.to_owned());
                }
            };
        }

        false
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Operator '{0}' cannot have the same name as a top-level production.")]
    OperatorNamedAsProduction(String),
    #[error("Operator '{0}' is defined in another production '{1}'.")]
    OperatorDefinedInAnotherProduction(String, String),
}
