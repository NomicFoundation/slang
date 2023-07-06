use std::collections::{HashMap, HashSet};

use crate::{
    types::{LanguageDefinitionRef, ParserRef, PrecedenceParserRef, ProductionRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, VersionSet, Visitor},
};

pub struct Parsers {
    language: LanguageDefinitionRef,

    current_production: Option<ProductionRef>,
    parser_to_production: HashMap<String, ProductionRef>,
    current_version_parsers: HashSet<String>,
}

impl Parsers {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let mut instance = Self {
            language: language.to_owned(),

            current_production: None,
            parser_to_production: HashMap::new(),
            current_version_parsers: HashSet::new(),
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Parsers {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_production = Some(production.to_owned());

        return true;
    }

    fn visit_version(
        &mut self,
        _version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_version_parsers.clear();

        return true;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        let parser_name = match &parser.name {
            Some(name) => name,
            None => return true,
        };

        if !self.validate_parser(parser_name, location, reporter) {
            return true;
        }

        if !self.current_version_parsers.insert(parser_name.to_owned()) {
            reporter.report(
                location,
                Errors::ParserDefinedTwiceInTheSameVersion(parser_name.to_owned()),
            );
        }

        return true;
    }

    fn visit_precedence_parser(
        &mut self,
        precedence_parser: &PrecedenceParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        for operator in &precedence_parser.operators {
            self.validate_parser(&operator.name, location, reporter);

            // Multiple operators can have the same name if under the same production.
        }

        return true;
    }
}

impl Parsers {
    fn validate_parser(
        &mut self,
        parser_name: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if self.language.productions.contains_key(parser_name) {
            reporter.report(
                location,
                Errors::ParserNamedAsProduction(parser_name.to_owned()),
            );
            return false;
        }

        let current_production = self.current_production.as_ref().unwrap();

        match self.parser_to_production.get(parser_name) {
            Some(existing_production) => {
                if current_production.name != existing_production.name {
                    reporter.report(
                        location,
                        Errors::ParserDefinedInAnotherProduction(
                            parser_name.to_owned(),
                            existing_production.name.to_owned(),
                        ),
                    );
                    return false;
                }
            }
            None => {
                self.parser_to_production
                    .insert(parser_name.to_owned(), current_production.to_owned());
            }
        };

        return true;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Parser '{0}' cannot have the same name as a top-level production.")]
    ParserNamedAsProduction(String),
    #[error("Parser '{0}' is defined in another production '{1}'.")]
    ParserDefinedInAnotherProduction(String, String),
    #[error("Parser '{0}' is defined twice in the same version.")]
    ParserDefinedTwiceInTheSameVersion(String),
}
