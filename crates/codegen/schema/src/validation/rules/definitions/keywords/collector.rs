use std::collections::HashMap;

use crate::types::{LanguageDefinitionRef, ProductionRef, ScannerDefinition, ScannerRef};
use crate::validation::visitors::{run_visitor, LocationRef, Reporter, Visitor};

pub struct KeywordsCollector {
    keywords: HashMap<String, ProductionRef>,
    current_production: Option<ProductionRef>,
}

impl KeywordsCollector {
    pub fn collect(
        language: &LanguageDefinitionRef,
        reporter: &mut Reporter,
    ) -> HashMap<String, ProductionRef> {
        let mut instance = Self {
            keywords: HashMap::new(),
            current_production: None,
        };

        run_visitor(&mut instance, language, reporter);

        instance.keywords
    }
}

impl Visitor for KeywordsCollector {
    fn visit_production(
        &mut self,
        production: &crate::types::ProductionRef,
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
        false
    }

    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        let ScannerDefinition::TrailingContext { scanner, .. } = &scanner.definition else {
            return true;
        };

        let Some(variations) = Self::collect_variations(scanner) else {
            return false;
        };

        let current_production = self.current_production.as_ref().unwrap();

        for variation in &variations {
            match self.keywords.get(variation) {
                Some(existing_production) => {
                    reporter.report(
                        location,
                        Errors::KeywordAlreadyDefined(
                            variation.to_owned(),
                            existing_production.name.to_owned(),
                        ),
                    );
                }
                None => {
                    self.keywords
                        .insert(variation.to_owned(), current_production.to_owned());
                }
            };
        }

        false
    }
}

impl KeywordsCollector {
    fn collect_variations(scanner: &ScannerRef) -> Option<Vec<String>> {
        // TODO: manually calculate variations until keywords are implemented.
        // Once that is done, report error if any scanners have variations, as they should be keywords.
        // https://github.com/NomicFoundation/slang/issues/505

        match &scanner.definition {
            ScannerDefinition::Choice(scanners) => {
                let mut variations = Vec::new();
                for scanner in scanners {
                    variations.extend(Self::collect_variations(scanner)?);
                }

                Some(variations)
            }
            ScannerDefinition::Difference { minuend, .. } => Self::collect_variations(minuend),
            ScannerDefinition::Optional(child) => {
                let mut variations = Self::collect_variations(child)?;
                variations.push(String::new());
                Some(variations)
            }
            ScannerDefinition::Range { from, to } => {
                let mut variations = Vec::new();
                for i in *from..=*to {
                    variations.push(i.to_string());
                }

                Some(variations)
            }
            ScannerDefinition::Sequence(children) => {
                let mut existing_variations = vec![];

                for child in children {
                    let new_variations = Self::collect_variations(child)?;

                    existing_variations = if existing_variations.is_empty() {
                        new_variations
                    } else {
                        let mut combined = vec![];

                        for existing in &existing_variations {
                            for new in &new_variations {
                                combined.push(format!("{existing}{new}"));
                            }
                        }

                        combined
                    }
                }

                Some(existing_variations)
            }
            ScannerDefinition::TrailingContext { scanner, .. } => Self::collect_variations(scanner),
            ScannerDefinition::Terminal(terminal) => {
                if terminal.chars().all(|c| c == '_' || c.is_alphanumeric()) {
                    Some(vec![terminal.to_owned()])
                } else {
                    None
                }
            }
            ScannerDefinition::Not(_)
            | ScannerDefinition::OneOrMore(_)
            | ScannerDefinition::Reference(_)
            | ScannerDefinition::ZeroOrMore(_) => {
                // Cannot be a keyword
                None
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Errors {
    #[error("Keyword '{0}' is already defined in production '{1}'.")]
    KeywordAlreadyDefined(String, String),
}
