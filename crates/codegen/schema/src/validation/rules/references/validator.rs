use crate::{
    types::{
        LanguageDefinitionRef, ParserDefinition, ParserRef, ProductionDefinition, ProductionRef,
        ScannerDefinition, ScannerRef,
    },
    validation::{
        rules::references::metadata::Metadata,
        visitors::{LocationRef, Reporter, VersionSet, Visitor},
    },
};

pub struct Validator {
    language: LanguageDefinitionRef,
    metadata: Metadata,
    current_production: Option<ProductionRef>,
    current_version_set: Option<VersionSet>,
}

impl Validator {
    pub fn new(language: &LanguageDefinitionRef, metadata: Metadata) -> Self {
        return Self {
            language: language.to_owned(),
            metadata,
            current_production: None,
            current_version_set: None,
        };
    }

    pub fn metadata(self) -> Metadata {
        return self.metadata;
    }
}

impl Visitor for Validator {
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
        version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_version_set = Some(version_set.to_owned());
        return true;
    }

    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if let ScannerDefinition::Reference(reference) = &scanner.definition {
            self.check_scanner_reference(&reference, location, reporter);
        }

        return true;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        match &parser.definition {
            ParserDefinition::Reference(reference) => {
                self.check_any_reference(reference, location, reporter);
            }
            ParserDefinition::DelimitedBy { open, close, .. } => {
                self.check_scanner_reference(&open.reference, location, reporter);
                self.check_scanner_reference(&close.reference, location, reporter);
            }
            ParserDefinition::SeparatedBy { separator, .. } => {
                self.check_scanner_reference(&separator.reference, location, reporter);
            }
            ParserDefinition::TerminatedBy { terminator, .. } => {
                self.check_scanner_reference(&terminator.reference, location, reporter);
            }
            _ => {}
        };

        return true;
    }
}

impl Validator {
    fn check_any_reference(
        &mut self,
        reference: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        match self.language.productions.get(reference) {
            Some(_) => {
                self.insert_reference(reference, location, reporter);
            }
            None => {
                reporter.report(location, Errors::NotDefined(reference.to_owned()));
            }
        };
    }

    fn check_scanner_reference(
        &mut self,
        reference: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        match self.language.productions.get(reference) {
            Some(production) => match production.definition {
                ProductionDefinition::Scanner { .. } => {
                    self.insert_reference(reference, location, reporter);
                }
                _ => {
                    reporter.report(location, Errors::MustBeScanner);
                }
            },
            None => {
                reporter.report(location, Errors::NotDefined(reference.to_owned()));
            }
        };
    }

    fn insert_reference(
        &mut self,
        reference: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        let production = self.current_production.as_ref().unwrap();
        let version_set = self.current_version_set.as_ref().unwrap();

        let can_be_added = self
            .metadata
            .add_reference(&production.name, &version_set, reference);

        if !can_be_added {
            reporter.report(
                &location,
                Errors::ReferenceVersionNotDefined(reference.to_owned(), version_set.to_owned()),
            );
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not defined anywhere in the grammar.")]
    NotDefined(String),
    #[error("A scanner may only reference other scanners.")]
    MustBeScanner,
    #[error("Production '{0}' is not fully defined in versions: {1}")]
    ReferenceVersionNotDefined(String, VersionSet),
}
