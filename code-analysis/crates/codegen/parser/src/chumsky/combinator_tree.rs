use std::rc::Rc;

use quote::quote;

use codegen_schema::*;

use super::{
    code_fragments::CodeFragments,
    combinator_forest::CombinatorForest,
    combinator_node::{CombinatorNode, CombinatorNodeRef, OperatorModel},
    naming, ProductionChumskyExtensions,
};

#[derive(Clone, Debug)]
pub struct CombinatorTree {
    // pub name: String,
    pub production: ProductionRef,
    pub root_node: CombinatorNodeRef,
}

pub type CombinatorTreeRef = Rc<CombinatorTree>;

impl CombinatorTree {
    pub fn from_production(grammar: &GrammarRef, production: &ProductionRef) -> CombinatorTreeRef {
        Rc::new(CombinatorTree {
            root_node: CombinatorNode::from_expression(
                grammar,
                production,
                &production.expression_to_generate(),
                Some(production.name.clone()),
            ),
            production: production.clone(),
        })
    }

    pub fn convert_to_expression_member(
        &mut self,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
    ) {
        self.root_node = {
            let ref this = self.root_node;
            let production = &self.production;
            if let CombinatorNode::Sequence { elements, .. } = this.as_ref() {
                let last_element_index = elements.len() - 1;

                let left =
                    if let CombinatorNode::Reference { production, .. } = elements[0].as_ref() {
                        production.name == parent.name
                    } else {
                        false
                    };

                let right = if let CombinatorNode::Reference { production, .. } =
                    elements[last_element_index].as_ref()
                {
                    production.name == parent.name
                } else {
                    false
                };

                let (operator, model) = match (&left, &right) {
                    (false, false) => (&elements[..], OperatorModel::None),
                    (false, true) => (&elements[..last_element_index], OperatorModel::UnaryPrefix),
                    (true, false) => (&elements[1..], OperatorModel::UnarySuffix),
                    (true, true) => (
                        &elements[1..last_element_index],
                        if production.expression_to_generate().config.associativity
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

                CombinatorNode::expression_member(
                    production.name.clone(),
                    parent,
                    next_sibling,
                    operator,
                    model,
                )
            } else {
                CombinatorNode::expression_member(
                    production.name.clone(),
                    parent,
                    next_sibling,
                    this.clone(),
                    OperatorModel::None,
                )
            }
        };
    }

    pub fn add_to_code_fragments(&self, forest: &CombinatorForest, code: &mut CodeFragments) {
        let parser_name = naming::to_parser_name_ident(&self.production.name);
        let field_name = naming::to_field_name_ident(&self.production.name);

        let (parser, result_type) = match self.production.kind {
            ProductionKind::Rule
            | ProductionKind::ExpressionRule
            | ProductionKind::ExpressionMemberRule => {
                code.add_rule_kind(self.production.name.clone());
                (
                    self.root_node.to_parser_code(forest, self, code),
                    quote!(cst::NodeRef),
                )
            }
            ProductionKind::Trivia => {
                code.add_rule_kind(self.production.name.clone());
                (
                    self.root_node.to_trivia_code(forest, self, code),
                    quote!(cst::NodeRef),
                )
            }
            ProductionKind::Token => {
                code.add_token_kind(self.production.name.clone());
                (self.root_node.to_lexer_code(code), quote!(lex::NodeRef))
            }
        };

        code.add_parser_predeclaration_fragment(
            quote!( let mut #parser_name = Recursive::<char, #result_type, ErrorType>::declare(); )
                .to_string(),
        );
        code.add_parser_definition_fragment(
            quote!( #parser_name.define(#parser.boxed()); ).to_string(),
        );
        code.add_parser_field_definition_fragment(
            quote!( pub #field_name: ParserType<#result_type>, ).to_string(),
        );

        code.add_parser_field_assignment_fragment(
            quote!( #field_name: #parser_name.then_ignore(end()).boxed(), ).to_string(),
        );
    }
}
