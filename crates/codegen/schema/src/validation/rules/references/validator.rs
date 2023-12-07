use crate::{
    types::{
        LanguageDefinitionRef, ParserDefinition, ParserRef, ProductionDefinition, ProductionRef,
        ScannerDefinition, ScannerRef,
    },
    validation::{
        rules::{references::metadata::Metadata, utils::is_a_keyword_scanner},
        visitors::{run_visitor, LocationRef, Reporter, VersionSet, Visitor},
    },
};

pub struct Validator<'validator> {
    language: LanguageDefinitionRef,
    metadata: &'validator mut Metadata,
    current_production: Option<ProductionRef>,
    current_version_set: Option<VersionSet>,
}

impl<'validator> Validator<'validator> {
    pub fn validate<'call: 'validator>(
        language: &'call LanguageDefinitionRef,
        metadata: &'call mut Metadata,
        reporter: &'call mut Reporter,
    ) {
        let mut instance = Self {
            language: language.to_owned(),
            metadata,
            current_production: None,
            current_version_set: None,
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Validator<'_> {
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
        version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.current_version_set = Some(version_set.to_owned());
        true
    }

    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if let ScannerDefinition::Reference(reference) = &scanner.definition {
            self.validate_reference(
                reference,
                ReferenceKind::ScannerToScanner,
                location,
                reporter,
            );
        }

        true
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        match &parser.definition {
            ParserDefinition::Reference(reference) => {
                self.validate_reference(
                    reference,
                    ReferenceKind::ParserToAnything,
                    location,
                    reporter,
                );
            }
            ParserDefinition::DelimitedBy { open, close, .. } => {
                self.validate_reference(
                    &open.reference,
                    ReferenceKind::ParserToScanner,
                    location,
                    reporter,
                );
                self.validate_reference(
                    &close.reference,
                    ReferenceKind::ParserToScanner,
                    location,
                    reporter,
                );
            }
            ParserDefinition::SeparatedBy { separator, .. } => {
                self.validate_reference(
                    &separator.reference,
                    ReferenceKind::ParserToScanner,
                    location,
                    reporter,
                );
            }
            ParserDefinition::TerminatedBy { terminator, .. } => {
                self.validate_reference(
                    &terminator.reference,
                    ReferenceKind::ParserToScanner,
                    location,
                    reporter,
                );
            }
            _ => {}
        };

        true
    }
}

#[derive(Clone, Copy)]
enum ReferenceKind {
    ParserToAnything,
    ParserToScanner,
    ScannerToScanner,
}

impl Validator<'_> {
    fn validate_reference(
        &mut self,
        reference_name: &str,
        validation_kind: ReferenceKind,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        let production = self.current_production.as_ref().unwrap();
        let version_set = self.current_version_set.as_ref().unwrap();

        if production.name == reference_name
            && !matches!(
                production.definition,
                ProductionDefinition::PrecedenceParser { .. }
            )
        {
            reporter.report(location, Errors::SelfReference(reference_name.to_owned()));
            return;
        }

        let Some(reference) = self.language.productions.get(reference_name) else {
            reporter.report(location, Errors::NotDefined(reference_name.to_owned()));
            return;
        };

        if !self.metadata.is_defined_over(reference_name, version_set) {
            reporter.report(
                location,
                Errors::ReferenceVersionNotDefined(
                    reference_name.to_owned(),
                    version_set.to_owned(),
                ),
            );
            return;
        }

        match validation_kind {
            ReferenceKind::ParserToAnything => {
                if reference.inlined
                    && matches!(reference.definition, ProductionDefinition::Scanner { .. })
                {
                    reporter.report(location, Errors::CannotBeInlined(reference_name.to_owned()));
                }
            }
            ReferenceKind::ParserToScanner => {
                if !matches!(reference.definition, ProductionDefinition::Scanner { .. }) {
                    reporter.report(location, Errors::MustBeScanner);
                } else if reference.inlined {
                    reporter.report(location, Errors::CannotBeInlined(reference_name.to_owned()));
                }
            }
            ReferenceKind::ScannerToScanner => {
                if !matches!(reference.definition, ProductionDefinition::Scanner { .. }) {
                    reporter.report(location, Errors::MustBeScanner);
                } else if !reference.inlined {
                    // Skip validation if this is a keyword
                    if !is_a_keyword_scanner(reference_name) {
                        reporter.report(location, Errors::MustBeInlined(reference_name.to_owned()));
                    }
                }
            }
        };

        self.metadata
            .add_reference(&production.name, version_set, reference_name);
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not defined anywhere in the grammar.")]
    NotDefined(String),
    #[error("Production '{0}' cannot reference itself.")]
    SelfReference(String),
    #[error("A scanner may only reference other scanners.")]
    MustBeScanner,
    #[error("Production '{0}' is not fully defined in versions: {1}")]
    ReferenceVersionNotDefined(String, VersionSet),
    #[error("Production '{0}' cannot be inlined to be valid here.")]
    CannotBeInlined(String),
    #[error("Production '{0}' must be inlined to be valid here.")]
    MustBeInlined(String),
}
