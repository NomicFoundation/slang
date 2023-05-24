use codegen_utils::errors::CodegenResult;
use indexmap::IndexSet;

use crate::{
    types::{ParserRef, PrecedenceParserRef, ProductionRef, SchemaRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub fn run(schema: &SchemaRef) -> CodegenResult<()> {
    let mut visitor = Definitions::new(schema);
    let mut reporter = Reporter::new();

    run_visitor(&mut visitor, schema, &mut reporter);

    return reporter.to_result();
}

struct Definitions {
    schema: SchemaRef,
    productions_so_far: IndexSet<String>,
}

impl Definitions {
    fn new(schema: &SchemaRef) -> Self {
        return Self {
            schema: schema.to_owned(),
            productions_so_far: IndexSet::new(),
        };
    }
}

impl Visitor for Definitions {
    fn visit_manifest(&mut self, location: &LocationRef, reporter: &mut Reporter) -> bool {
        let required_productions = self.schema.required_productions();

        for name in required_productions {
            if !self.schema.productions.contains_key(name) {
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
        let name = production.name();
        if !self.productions_so_far.insert(name.to_owned()) {
            reporter.report(location, Errors::DuplicateProduction(name.to_owned()));
        }

        return true;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if let Some(name) = &parser.name {
            if self.schema.productions.contains_key(name) {
                reporter.report(
                    location,
                    Errors::ExpressionNamedAsProduction(name.to_owned()),
                );
            }
        }

        return true;
    }

    fn visit_precedence_parser(
        &mut self,
        precedence_parser: &PrecedenceParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if let Some(name) = &precedence_parser.name {
            if self.schema.productions.contains_key(name) {
                reporter.report(
                    location,
                    Errors::ExpressionNamedAsProduction(name.to_owned()),
                );
            }
        }

        return true;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Required production '{0}' is not defined.")]
    MissingRequired(String),
    #[error("Production '{0}' is defined more than once.")]
    DuplicateProduction(String),
    #[error("Expression '{0}' cannot have the same name as a top-level production.")]
    ExpressionNamedAsProduction(String),
}
