use std::collections::HashMap;

use crate::types::{LanguageDefinitionRef, ProductionRef, ScannerDefinition, ScannerRef};
use crate::validation::rules::utils::is_a_keyword_scanner;
use crate::validation::visitors::{run_visitor, LocationRef, Reporter, Visitor};

pub struct KeywordsValidator {
    keywords: HashMap<String, ProductionRef>,
    current_production: Option<ProductionRef>,
}

impl KeywordsValidator {
    pub fn validate(
        language: &LanguageDefinitionRef,
        keywords: HashMap<String, ProductionRef>,
        reporter: &mut Reporter,
    ) {
        let mut instance = Self {
            keywords,
            current_production: None,
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for KeywordsValidator {
    fn visit_production(
        &mut self,
        production: &crate::types::ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_production = Some(production.to_owned());

        // Skip validation if this is a keyword:
        !is_a_keyword_scanner(&production.name)
    }

    fn visit_parser(
        &mut self,
        _parser: &crate::types::ParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        false
    }

    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if let ScannerDefinition::Terminal(terminal) = &scanner.definition {
            if let Some(production) = self.keywords.get(terminal) {
                if production.name != self.current_production.as_ref().unwrap().name {
                    reporter.report(
                        location,
                        Errors::ShouldReferenceExistingKeyword(production.name.to_owned()),
                    );
                }
            }
        }

        true
    }

    fn visit_precedence_parser(
        &mut self,
        _precedence_parser: &crate::types::PrecedenceParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        false
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Errors {
    #[error("You should reference the existing keyword '{0}' instead.")]
    ShouldReferenceExistingKeyword(String),
}
