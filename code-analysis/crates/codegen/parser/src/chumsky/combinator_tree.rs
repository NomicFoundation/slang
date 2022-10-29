use std::rc::Rc;

use quote::quote;

use codegen_schema::*;
use semver::Version;

use super::{
    code_fragments::CodeFragments,
    combinator_forest::CombinatorForest,
    combinator_node::{CombinatorNode, CombinatorNodeRef, OperatorModel},
    production::ProductionChumskyExtensions,
};

#[derive(Clone, Debug)]
pub struct CombinatorTree {
    // pub name: String,
    pub production: ProductionRef,
    pub root_node: CombinatorNodeRef,
}

pub type CombinatorTreeRef = Rc<CombinatorTree>;

impl CombinatorTree {
    pub fn from_production(
        grammar: &GrammarRef,
        production: &ProductionRef,
        version: &Version,
    ) -> CombinatorTreeRef {
        Rc::new(CombinatorTree {
            root_node: CombinatorNode::from_expression(
                grammar,
                production,
                version,
                &production.expression_for_version(version),
                Some(production.name.clone()),
            ),
            production: production.clone(),
        })
    }

    pub fn convert_to_precedence_rule_member(
        &mut self,
        parent_production: ProductionRef,
        version: &Version,
        next_sibling: Option<ProductionRef>,
    ) {
        self.root_node = {
            let ref this = self.root_node;
            let production = &self.production;
            if let CombinatorNode::Sequence { elements, .. } = this.as_ref() {
                let last_element_index = elements.len() - 1;

                let left =
                    if let CombinatorNode::Reference { production, .. } = elements[0].as_ref() {
                        production.name == parent_production.name
                    } else {
                        false
                    };

                let right = if let CombinatorNode::Reference { production, .. } =
                    elements[last_element_index].as_ref()
                {
                    production.name == parent_production.name
                } else {
                    false
                };

                let (operator, model) = match (&left, &right) {
                    (false, false) => (&elements[..], OperatorModel::None),
                    (false, true) => (&elements[..last_element_index], OperatorModel::UnaryPrefix),
                    (true, false) => (&elements[1..], OperatorModel::UnarySuffix),
                    (true, true) => (
                        &elements[1..last_element_index],
                        if production
                            .expression_for_version(version)
                            .config
                            .associativity
                            == Some(ExpressionAssociativity::Right)
                        {
                            OperatorModel::BinaryRightAssociative
                        } else {
                            OperatorModel::BinaryLeftAssociative
                        },
                    ),
                };
                let operator = if operator.len() == 1 {
                    operator[0].clone()
                } else {
                    CombinatorNode::sequence(None, operator.into())
                };

                CombinatorNode::precedence_rule_member(
                    production.name.clone(),
                    parent_production,
                    next_sibling,
                    operator,
                    model,
                )
            } else {
                CombinatorNode::precedence_rule_member(
                    production.name.clone(),
                    parent_production,
                    next_sibling,
                    this.clone(),
                    OperatorModel::None,
                )
            }
        };
    }

    pub fn add_to_code_fragments(&self, forest: &CombinatorForest, code: &mut CodeFragments) {
        match self.production.kind {
            ProductionKind::Rule => {
                code.add_rule_kind(self.production.name.clone());
                let parser = self.root_node.to_parser_code(forest, self, false, code);
                code.add_parser(
                    self.production.name.clone(),
                    &forest.version,
                    parser,
                    quote!(cst::NodeRef),
                );
            }

            ProductionKind::Trivia => {
                code.add_rule_kind(self.production.name.clone());
                let parser = self.root_node.to_parser_code(forest, self, true, code);
                code.add_parser(
                    self.production.name.clone(),
                    &forest.version,
                    parser,
                    quote!(cst::NodeRef),
                );
            }

            ProductionKind::Token => {
                code.add_token_kind(self.production.name.clone());
                let parser = self.root_node.to_lexer_code(code);
                code.add_parser(
                    self.production.name.clone(),
                    &forest.version,
                    parser,
                    quote!(lex::NodeRef),
                );
            }
        };
    }
}
