use super::*;

pub trait GrammarVisitor {
    fn grammar_enter(&mut self, _grammar: &Grammar) {}
    fn grammar_leave(&mut self, _grammar: &Grammar) {}

    fn grammar_element_enter(&mut self, _element: &GrammarElement) {}
    fn grammar_element_leave(&mut self, _element: &GrammarElement) {}

    fn scanner_definition_enter(&mut self, _scanner: &ScannerDefinitionRef) {}
    fn scanner_definition_leave(&mut self, _scanner: &ScannerDefinitionRef) {}
    fn trivia_parser_definition_enter(&mut self, _trivia_parser: &TriviaParserDefinitionRef) {}
    fn trivia_parser_definition_leave(&mut self, _trivia_parser: &TriviaParserDefinitionRef) {}
    fn parser_definition_enter(&mut self, _parser: &ParserDefinitionRef) {}
    fn parser_definition_leave(&mut self, _parser: &ParserDefinitionRef) {}
    fn precedence_parser_definition_enter(
        &mut self,
        _precedence_parser: &PrecedenceParserDefinitionRef,
    ) {
    }
    fn precedence_parser_definition_leave(
        &mut self,
        _precedence_parser: &PrecedenceParserDefinitionRef,
    ) {
    }

    fn scanner_definition_node_enter(&mut self, _node: &ScannerDefinitionNode) {}
    fn scanner_definition_node_leave(&mut self, _node: &ScannerDefinitionNode) {}
    fn parser_definition_node_enter(&mut self, _node: &ParserDefinitionNode) {}
    fn parser_definition_node_leave(&mut self, _node: &ParserDefinitionNode) {}
    fn precedence_parser_definition_node_enter(&mut self, _node: &PrecedenceParserDefinitionNode) {}
    fn precedence_parser_definition_node_leave(&mut self, _node: &PrecedenceParserDefinitionNode) {}
}

pub trait Visitable {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V);
}
