use std::collections::{HashMap, HashSet};

use crate::{
    types::{LanguageDefinitionRef, ParserRef, PrecedenceParserRef, ProductionRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, VersionSet, Visitor},
};
use codegen_utils::errors::CodegenResult;

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    let mut instance = Definitions {
        language: language.to_owned(),

        productions_already_seen: HashSet::new(),
        current_production: None,

        parser_to_production: HashMap::new(),
        current_version_parsers: HashSet::new(),
    };

    run_visitor(&mut instance, language, &mut reporter);

    return reporter.to_result();
}

struct Definitions {
    language: LanguageDefinitionRef,

    productions_already_seen: HashSet<String>,
    current_production: Option<ProductionRef>,

    parser_to_production: HashMap<String, ProductionRef>,
    current_version_parsers: HashSet<String>,
}

impl Visitor for Definitions {
    fn visit_manifest(&mut self, location: &LocationRef, reporter: &mut Reporter) -> bool {
        let required_productions = self.language.required_productions();

        for name in required_productions {
            if let Some(production) = self.language.productions.get(name) {
                if production.inlined {
                    reporter.report(&location, Errors::RequiredCannotBeInlined(name.to_owned()));
                }
            } else {
                reporter.report(&location, Errors::MissingRequired(name.to_owned()));
            }
        }

        return true;
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

        if !self.check_parser(parser_name, location, reporter) {
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
            self.check_parser(&operator.name, location, reporter);

            // Multiple operators can have the same name if under the same production.
        }

        return true;
    }
}

impl Definitions {
    fn check_parser(
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
    #[error("Required production '{0}' is not defined.")]
    MissingRequired(String),
    #[error("Required production '{0}' cannot be inlined.")]
    RequiredCannotBeInlined(String),
    #[error("Production '{0}' is defined more than once.")]
    DuplicateProduction(String),
    #[error("Parser '{0}' cannot have the same name as a top-level production.")]
    ParserNamedAsProduction(String),
    #[error("Parser '{0}' is defined in another production '{1}'.")]
    ParserDefinedInAnotherProduction(String, String),
    #[error("Parser '{0}' is defined twice in the same version.")]
    ParserDefinedTwiceInTheSameVersion(String),
}
