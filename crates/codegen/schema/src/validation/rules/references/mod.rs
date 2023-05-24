use codegen_utils::errors::CodegenResult;
use indexmap::{IndexMap, IndexSet};

use crate::{
    types::{
        ParserDefinition, ParserRef, Production, ProductionRef, ScannerDefinition, ScannerRef,
        SchemaRef,
    },
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub fn run(schema: &SchemaRef) -> CodegenResult<()> {
    let mut visitor = References::new(schema);
    let mut reporter = Reporter::new();

    run_visitor(&mut visitor, schema, &mut reporter);

    return reporter.to_result();
}

struct References {
    schema: SchemaRef,
    definitions: IndexMap<String, LocationRef>,
    references: IndexSet<String>,
}

impl References {
    fn new(schema: &SchemaRef) -> Self {
        return Self {
            schema: schema.to_owned(),
            definitions: IndexMap::new(),
            references: IndexSet::new(),
        };
    }
}

impl Visitor for References {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.definitions
            .insert(production.name().to_owned(), location.clone());

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
                self.check_any_reference(&reference, location, reporter);
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

    fn finish(&mut self, reporter: &mut Reporter) {
        self.check_not_used(reporter);
    }
}

impl References {
    fn check_scanner_reference(
        &mut self,
        reference: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        match self.schema.productions.get(reference) {
            Some(production) => match production.as_ref() {
                Production::Scanner { .. } => {
                    self.references.insert(reference.to_owned());
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

    fn check_any_reference(
        &mut self,
        reference: &str,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) {
        match self.schema.productions.get(reference) {
            Some(_) => {
                self.references.insert(reference.to_owned());
            }
            None => {
                reporter.report(location, Errors::NotDefined(reference.to_owned()));
            }
        };
    }

    fn check_not_used(&self, reporter: &mut Reporter) {
        let required_productions = self.schema.required_productions();

        for (name, location) in &self.definitions {
            if required_productions.contains(name.as_str()) {
                continue;
            }

            if !self.references.contains(name) {
                reporter.report(location, Errors::NotUsed(name.to_owned()));
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not defined anywhere in the grammar.")]
    NotDefined(String),
    #[error("A scanner may only reference other scanners.")]
    MustBeScanner,
    #[error("Production '{0}' is not used anywhere in the grammar.")]
    NotUsed(String),
}
