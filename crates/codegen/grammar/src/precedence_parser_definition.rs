use std::rc::Rc;

use super::*;

#[allow(dead_code)]
pub trait PrecedenceParserDefinition
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> &'static str;
    fn source_location(&self) -> SourceLocation;
    fn node(&self) -> &PrecedenceParserDefinitionNode;
    fn context(&self) -> &'static str;
}

pub type PrecedenceParserDefinitionRef = Rc<dyn PrecedenceParserDefinition>;

impl Visitable for PrecedenceParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.precedence_parser_definition_enter(self);
        self.node().accept_visitor(visitor);
        visitor.precedence_parser_definition_leave(self);
    }
}

#[derive(Clone, Debug)]
pub struct PrecedenceParserDefinitionNode {
    pub primary_expression: Box<ParserDefinitionNode>,
    pub operators: Vec<(
        Vec<VersionQualityRange>,
        PrecedenceOperatorModel,
        &'static str, // name
        ParserDefinitionRef,
    )>,
    pub source_location: SourceLocation,
}

impl Visitable for PrecedenceParserDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.precedence_parser_definition_node_enter(self);
        self.primary_expression.accept_visitor(visitor);
        visitor.precedence_parser_definition_node_leave(self);
    }
}

#[derive(Clone, Debug)]
pub enum PrecedenceOperatorModel {
    BinaryLeftAssociative,
    BinaryRightAssociative,
    Prefix,
    Postfix,
}
