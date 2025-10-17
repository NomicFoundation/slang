//! Definitions of the [`GrammarElement`]s and the grammar itself ([`Grammar`]) used by the parser generator.

use std::collections::HashMap;
use std::rc::Rc;

use language_definition::model::{self, Identifier};

pub mod parser_definition;
pub mod precedence_parser_definition;
pub mod resolver;
pub mod scanner_definition;
pub mod visitor;

pub use parser_definition::*;
pub use precedence_parser_definition::*;
pub use resolver::ResolveCtx;
pub use scanner_definition::*;
pub use visitor::*;

pub struct Grammar {
    pub elements: HashMap<Identifier, GrammarElement>,
}

impl Grammar {
    pub fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.grammar_enter(self);
        for element in self.elements.values() {
            element.accept_visitor(visitor);
        }
        visitor.grammar_leave(self);
    }
}

#[allow(clippy::enum_variant_names)] // this will be removed soon
#[derive(Clone, strum_macros::EnumTryAs)]
pub enum GrammarElement {
    ScannerDefinition(ScannerDefinitionRef),
    KeywordScannerDefinition(Rc<model::KeywordItem>),
    TriviaParserDefinition(TriviaParserDefinitionRef),
    ParserDefinition(ParserDefinitionRef),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef),
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
            Self::KeywordScannerDefinition(_) => {}
            Self::TriviaParserDefinition(trivia_parser) => trivia_parser.accept_visitor(visitor),
            Self::ParserDefinition(parser) => parser.accept_visitor(visitor),
            Self::PrecedenceParserDefinition(precedence_parser) => {
                precedence_parser.accept_visitor(visitor);
            }
        }
    }
}
