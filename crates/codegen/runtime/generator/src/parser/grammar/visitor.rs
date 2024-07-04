use crate::parser::grammar::{
    Grammar, ParserDefinitionNode, ParserDefinitionRef, PrecedenceParserDefinitionNode,
    PrecedenceParserDefinitionRef, ScannerDefinitionRef, TriviaParserDefinitionRef,
};

pub trait GrammarVisitor {
    fn grammar_enter(&mut self, _grammar: &Grammar) {}
    fn grammar_leave(&mut self, _grammar: &Grammar) {}

    fn scanner_definition_enter(&mut self, _scanner: &ScannerDefinitionRef) {}
    fn trivia_parser_definition_enter(&mut self, _trivia_parser: &TriviaParserDefinitionRef) {}
    fn parser_definition_enter(&mut self, _parser: &ParserDefinitionRef) {}
    fn precedence_parser_definition_enter(&mut self, _parser: &PrecedenceParserDefinitionRef) {}

    fn parser_definition_node_enter(&mut self, _node: &ParserDefinitionNode) {}
    fn precedence_parser_definition_node_enter(&mut self, _node: &PrecedenceParserDefinitionNode) {}
}

pub trait Visitable {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V);
}
