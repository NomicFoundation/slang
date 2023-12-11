use crate::types::{
    LanguageDefinitionRef, ParserDefinition, ParserRef, ProductionDefinition, ProductionRef,
    ScannerRef, VersionMap,
};
use crate::validation::visitors::{run_visitor, LocationRef, Reporter, Visitor};

pub struct ConsistentShape {
    language: LanguageDefinitionRef,

    current_production: Option<ProductionRef>,
    is_root_parser: bool,
}

impl ConsistentShape {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let mut instance = Self {
            language: language.to_owned(),

            current_production: None,
            is_root_parser: false,
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for ConsistentShape {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_production = Some(production.to_owned());
        true
    }

    fn visit_version(
        &mut self,
        _version_set: &crate::validation::visitors::VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.is_root_parser = true;
        true
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        let must_be_single_named = if self.is_root_parser {
            self.current_production.as_ref().unwrap().inlined
        } else {
            true
        };

        self.is_root_parser = false;

        if must_be_single_named && !self.is_single_named(parser) {
            reporter.report(location, Errors::MustProduceSingleNamed);
        }

        true
    }

    fn visit_scanner(
        &mut self,
        _scanner: &ScannerRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        false
    }
}

impl ConsistentShape {
    fn is_single_named(&self, parser: &ParserRef) -> bool {
        match &parser.definition {
            ParserDefinition::Choice(_)
            | ParserDefinition::DelimitedBy { .. }
            | ParserDefinition::Sequence(_)
            | ParserDefinition::TerminatedBy { .. } => {
                // Constant number of children. Doesn't have to be named:
                true
            }

            ParserDefinition::OneOrMore(_)
            | ParserDefinition::SeparatedBy { .. }
            | ParserDefinition::ZeroOrMore(_) => {
                // Variable number of children. Must be named:
                false
            }

            ParserDefinition::Optional(optional) => {
                // Optional doesn't have to be named, but its child must be.
                // Otherwise, it will be indistinguishable from the parent's other children.
                self.is_single_named(optional)
            }

            ParserDefinition::Reference(reference) => {
                let production = &self.language.productions[reference];
                if !production.inlined {
                    // will always produce a named node:
                    return true;
                }

                match &production.definition {
                    ProductionDefinition::Scanner { .. } => {
                        // If not inlined, it will always produce a named node:
                        !production.inlined
                    }

                    ProductionDefinition::Parser { version_map }
                    | ProductionDefinition::TriviaParser { version_map } => match &version_map {
                        VersionMap::Unversioned(parser) => self.is_single_named(parser),
                        VersionMap::Versioned(versioned) => {
                            versioned.values().all(|parser| match &parser {
                                Some(parser) => self.is_single_named(parser),
                                None => true,
                            })
                        }
                    },

                    ProductionDefinition::PrecedenceParser { version_map } => match &version_map {
                        VersionMap::Unversioned(parser) => {
                            self.is_single_named(&parser.primary_expression)
                        }
                        VersionMap::Versioned(versioned) => {
                            versioned.values().all(|parser| match &parser {
                                Some(parser) => self.is_single_named(&parser.primary_expression),
                                None => true,
                            })
                        }
                    },
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("This parser should always produce a single named node. Otherwise, it affects its parent children count.")]
    MustProduceSingleNamed,
}
