use std::collections::{BTreeSet, HashMap};

use semver::Version;

use super::*;

pub struct Grammar {
    pub name: String,
    pub versions: BTreeSet<Version>,
    pub leading_trivia_parser: TriviaParserDefinitionRef,
    pub trailing_trivia_parser: TriviaParserDefinitionRef,
    pub elements: HashMap<&'static str, GrammarElement>,
}

impl Grammar {
    pub fn elements(&self) -> &HashMap<&'static str, GrammarElement> {
        &self.elements
    }

    pub fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.grammar_enter(self);
        for element in self.elements.values() {
            element.accept_visitor(visitor);
        }
        visitor.grammar_leave(self);
    }

    pub fn register<E: Into<GrammarElement>>(&mut self, instance: E) {
        let element: GrammarElement = instance.into();
        self.elements.insert(element.name(), element);
    }
}

#[derive(Clone)]
pub enum GrammarElement {
    ScannerDefinition(ScannerDefinitionRef),
    TriviaParserDefinition(TriviaParserDefinitionRef),
    ParserDefinition(ParserDefinitionRef),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef),
}

impl GrammarElement {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ScannerDefinition(scanner) => scanner.name(),
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.name(),
            Self::ParserDefinition(parser) => parser.name(),
            Self::PrecedenceParserDefinition(precedence_parser) => precedence_parser.name(),
        }
    }

    pub fn source_location(&self) -> SourceLocation {
        match self {
            Self::ScannerDefinition(scanner) => scanner.source_location(),
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.source_location(),
            Self::ParserDefinition(parser) => parser.source_location(),
            Self::PrecedenceParserDefinition(precedence_parser) => {
                precedence_parser.source_location()
            }
        }
    }
}

impl Into<GrammarElement> for ScannerDefinitionRef {
    fn into(self) -> GrammarElement {
        GrammarElement::ScannerDefinition(self)
    }
}

impl Into<GrammarElement> for TriviaParserDefinitionRef {
    fn into(self) -> GrammarElement {
        GrammarElement::TriviaParserDefinition(self)
    }
}

impl Into<GrammarElement> for ParserDefinitionRef {
    fn into(self) -> GrammarElement {
        GrammarElement::ParserDefinition(self)
    }
}

impl Into<GrammarElement> for PrecedenceParserDefinitionRef {
    fn into(self) -> GrammarElement {
        GrammarElement::PrecedenceParserDefinition(self)
    }
}

impl Visitable for GrammarElement {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.grammar_element_enter(self);
        match self {
            Self::ScannerDefinition(scanner) => scanner.accept_visitor(visitor),
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.accept_visitor(visitor),
            Self::ParserDefinition(parser) => parser.accept_visitor(visitor),
            Self::PrecedenceParserDefinition(precedence_parser) => {
                precedence_parser.accept_visitor(visitor)
            }
        }
        visitor.grammar_element_leave(self);
    }
}
