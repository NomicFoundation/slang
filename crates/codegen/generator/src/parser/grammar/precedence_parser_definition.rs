use std::fmt::Debug;
use std::rc::Rc;

use language_definition::model::Identifier;
pub use language_definition::model::OperatorModel as PrecedenceOperatorModel;

use crate::parser::grammar::{GrammarVisitor, ParserDefinitionNode, Visitable};

pub trait PrecedenceParserDefinition: Debug {
    fn name(&self) -> &Identifier;
    fn node(&self) -> &PrecedenceParserDefinitionNode;
    fn context(&self) -> &Identifier;
}

pub type PrecedenceParserDefinitionRef = Rc<dyn PrecedenceParserDefinition>;

impl Visitable for PrecedenceParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.precedence_parser_definition_enter(self);
        self.node().accept_visitor(visitor);
    }
}

#[derive(Clone, Debug)]
pub struct PrecedenceParserDefinitionNode {
    pub primary_expression: Box<ParserDefinitionNode>,
    pub operators: Vec<(PrecedenceOperatorModel, Identifier, ParserDefinitionNode)>,
    pub precedence_expression_names: Vec<Identifier>,
}

impl Visitable for PrecedenceParserDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.precedence_parser_definition_node_enter(self);
        self.primary_expression.accept_visitor(visitor);
    }
}
