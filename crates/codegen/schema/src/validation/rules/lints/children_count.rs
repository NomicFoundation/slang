use crate::{
    types::{
        LanguageDefinitionRef, ParserDefinition, ParserRef, PrecedenceParserRef, ScannerDefinition,
        ScannerRef,
    },
    validation::visitors::{run_visitor, LocationRef, Reporter, Visitor},
};

pub struct ChildrenCount;

impl ChildrenCount {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        run_visitor(&mut Self, language, reporter);
    }
}

impl Visitor for ChildrenCount {
    fn visit_scanner(
        &mut self,
        scanner: &ScannerRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        match &scanner.definition {
            ScannerDefinition::Choice(children) | ScannerDefinition::Sequence(children) => {
                if children.len() < 2 {
                    reporter.report(location, Errors::MinChildrenCount(2));
                }
            }
            ScannerDefinition::Terminal(terminal) => {
                if terminal.is_empty() {
                    reporter.report(location, Errors::EmptyTerminal);
                }
            }
            ScannerDefinition::Difference { .. }
            | ScannerDefinition::Not(_)
            | ScannerDefinition::OneOrMore(_)
            | ScannerDefinition::Optional(_)
            | ScannerDefinition::Range { .. }
            | ScannerDefinition::Reference(_)
            | ScannerDefinition::TrailingContext { .. }
            | ScannerDefinition::ZeroOrMore(_) => {}
        };

        return true;
    }

    fn visit_parser(
        &mut self,
        parser: &ParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        match &parser.definition {
            ParserDefinition::Choice(children) | ParserDefinition::Sequence(children) => {
                if parser.name.is_none() && children.len() < 2 {
                    reporter.report(location, Errors::MinChildrenCount(2));
                }
            }
            ParserDefinition::DelimitedBy { .. }
            | ParserDefinition::OneOrMore(_)
            | ParserDefinition::Optional(_)
            | ParserDefinition::Reference(_)
            | ParserDefinition::SeparatedBy { .. }
            | ParserDefinition::TerminatedBy { .. }
            | ParserDefinition::ZeroOrMore(_) => {}
        };

        return true;
    }

    fn visit_precedence_parser(
        &mut self,
        precedence_parser: &PrecedenceParserRef,
        location: &LocationRef,
        reporter: &mut Reporter,
    ) -> bool {
        if precedence_parser.operators.is_empty() {
            reporter.report(location, Errors::MinChildrenCount(1));
        }

        return true;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Must have at least {0} children.")]
    MinChildrenCount(usize),
    #[error("A terminal cannot be empty.")]
    EmptyTerminal,
}
