use std::collections::{BTreeSet, HashMap};

use semver::Version;

pub mod constructor;
pub mod parser_definition;
pub mod precedence_parser_definition;
pub mod scanner_definition;
pub mod version_quality;
pub mod visitor;

pub use constructor::GrammarConstructorDslV2;
pub use parser_definition::*;
pub use precedence_parser_definition::*;
pub use scanner_definition::*;
pub use version_quality::*;
pub use visitor::*;

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
    KeywordScannerDefinition(KeywordScannerDefinitionRef),
    TriviaParserDefinition(TriviaParserDefinitionRef),
    ParserDefinition(ParserDefinitionRef),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef),
}

impl GrammarElement {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ScannerDefinition(scanner) => scanner.name(),
            Self::KeywordScannerDefinition(scanner) => scanner.name(),
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.name(),
            Self::ParserDefinition(parser) => parser.name(),
            Self::PrecedenceParserDefinition(precedence_parser) => precedence_parser.name(),
        }
    }
}

impl From<ScannerDefinitionRef> for GrammarElement {
    fn from(def: ScannerDefinitionRef) -> Self {
        GrammarElement::ScannerDefinition(def)
    }
}

impl From<TriviaParserDefinitionRef> for GrammarElement {
    fn from(def: TriviaParserDefinitionRef) -> Self {
        GrammarElement::TriviaParserDefinition(def)
    }
}

impl From<ParserDefinitionRef> for GrammarElement {
    fn from(def: ParserDefinitionRef) -> Self {
        GrammarElement::ParserDefinition(def)
    }
}

impl From<PrecedenceParserDefinitionRef> for GrammarElement {
    fn from(def: PrecedenceParserDefinitionRef) -> Self {
        GrammarElement::PrecedenceParserDefinition(def)
    }
}

impl Visitable for GrammarElement {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        match self {
            Self::ScannerDefinition(scanner) => scanner.accept_visitor(visitor),
            Self::KeywordScannerDefinition(scanner) => scanner.accept_visitor(visitor),
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.accept_visitor(visitor),
            Self::ParserDefinition(parser) => parser.accept_visitor(visitor),
            Self::PrecedenceParserDefinition(precedence_parser) => {
                precedence_parser.accept_visitor(visitor);
            }
        }
    }
}
