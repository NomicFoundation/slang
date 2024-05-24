use crate::parser::grammar::{
    KeywordScannerDefinitionRef, ParserDefinitionNode, ParserDefinitionRef,
    PrecedenceParserDefinitionRef, ScannerDefinitionNode, ScannerDefinitionRef,
    TriviaParserDefinitionRef,
};

pub trait GrammarVisitor {
    fn scanner_definition_enter(&mut self, _scanner: &ScannerDefinitionRef) {}
    fn keyword_scanner_definition_enter(&mut self, _scanner: &KeywordScannerDefinitionRef) {}
    fn trivia_parser_definition_enter(&mut self, _trivia_parser: &TriviaParserDefinitionRef) {}
    fn parser_definition_enter(&mut self, _parser: &ParserDefinitionRef) {}
    fn precedence_parser_definition_enter(&mut self, _parser: &PrecedenceParserDefinitionRef) {}

    fn scanner_definition_node_enter(&mut self, _node: &ScannerDefinitionNode) {}
    fn parser_definition_node_enter(&mut self, _node: &ParserDefinitionNode) {}
}

pub trait Visitable {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V);
}
