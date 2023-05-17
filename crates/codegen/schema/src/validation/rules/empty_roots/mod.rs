use codegen_utils::errors::CodegenResult;

use crate::{
    types::{ParserDefinition, ParserRef, ProductionRef, ScannerDefinition, ScannerRef, SchemaRef},
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub fn run(schema: &SchemaRef) -> CodegenResult<()> {
    let mut visitor = EmptyRoots::new(schema);
    let mut reporter = Reporter::new();

    run_visitor(&mut visitor, schema, &mut reporter);

    return reporter.to_result();
}

struct EmptyRoots {
    schema: SchemaRef,
}

impl EmptyRoots {
    fn new(schema: &SchemaRef) -> Self {
        return Self {
            schema: schema.to_owned(),
        };
    }
}

impl Visitor for EmptyRoots {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        if production.name() == &self.schema.root_production {
            // Skip, as it is allowed to be empty.
            return false;
        }

        return true;
    }

    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if possible_empty_scanner(scanner) {
            reporter.report(location, Errors::EmptyRoot);
        }

        // Only check the top-most expression. Ignore nested ones.
        return false;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if possible_empty_parser(parser) {
            reporter.report(location, Errors::EmptyRoot);
        }

        // Only check the top-most expression. Ignore nested ones.
        return false;
    }
}

fn possible_empty_scanner(scanner: &ScannerRef) -> bool {
    match &scanner.definition {
        ScannerDefinition::Choice(choices) => {
            return choices.iter().any(possible_empty_scanner);
        }

        ScannerDefinition::Difference {
            minuend,
            subtrahend,
        } => {
            return possible_empty_scanner(minuend) || possible_empty_scanner(subtrahend);
        }

        ScannerDefinition::OneOrMore(expression)
        | ScannerDefinition::TrailingContext { expression, .. } => {
            return possible_empty_scanner(expression);
        }

        ScannerDefinition::Optional(_) | ScannerDefinition::ZeroOrMore(_) => {
            return true;
        }

        ScannerDefinition::Sequence(sequence) => {
            return sequence.iter().all(possible_empty_scanner);
        }

        ScannerDefinition::Repeat {
            min, expression, ..
        } => {
            if *min == 0 {
                return true;
            }

            return possible_empty_scanner(expression);
        }

        ScannerDefinition::Not { .. }
        | ScannerDefinition::Range { .. }
        | ScannerDefinition::Reference(_)
        | ScannerDefinition::Terminal(_) => {
            return false;
        }
    };
}

fn possible_empty_parser(parser: &ParserRef) -> bool {
    match &parser.definition {
        ParserDefinition::Choice(choices) => {
            return choices.iter().any(possible_empty_parser);
        }

        ParserDefinition::OneOrMore(expression)
        | ParserDefinition::SeparatedBy { expression, .. }
        | ParserDefinition::TerminatedBy { expression, .. } => {
            return possible_empty_parser(expression);
        }

        ParserDefinition::Optional(_) | ParserDefinition::ZeroOrMore(_) => {
            return true;
        }

        ParserDefinition::Sequence(sequence) => {
            return sequence.iter().all(possible_empty_parser);
        }

        ParserDefinition::Repeat {
            min, expression, ..
        } => {
            if *min == 0 {
                return true;
            }

            return possible_empty_parser(expression);
        }

        ParserDefinition::DelimitedBy { .. } | ParserDefinition::Reference(_) => {
            return false;
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error(
        "Root expression cannot be optionally empty. Refactor usages to specify the arity instead."
    )]
    EmptyRoot,
}
