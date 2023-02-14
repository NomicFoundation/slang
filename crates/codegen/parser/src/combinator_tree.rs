use std::cell::Cell;

use codegen_ebnf::ProductionEBNFGeneratorExtensions;
use codegen_schema::types::productions::{
    ExpressionAssociativity, ExpressionRef, ProductionKind, ProductionRef, ProductionVersioning,
};

use crate::first_set::FirstSet;

use super::{
    code_generator::CodeGenerator,
    combinator_context::CombinatorContext,
    combinator_node::{CombinatorNode, OperatorModel, PrecedenceRuleOperator},
};

pub struct CombinatorTree<'context> {
    pub context: &'context CombinatorContext<'context>,
    pub production: ProductionRef,
    pub root_node: Cell<Option<&'context CombinatorNode<'context>>>,
}

impl<'context> CombinatorTree<'context> {
    pub fn new(
        context: &'context CombinatorContext<'context>,
        production: &ProductionRef,
    ) -> &'context CombinatorTree<'context> {
        context.alloc_tree(CombinatorTree {
            context: context,
            production: production.clone(),
            root_node: Cell::new(None),
        })
    }

    pub fn first_set(&self) -> FirstSet {
        self.root_node.get().unwrap().first_set()
    }

    pub fn ensure_tree_is_built(&'context self) {
        if self.root_node.get().is_none() {
            let expression = &self.expression();
            let node = CombinatorNode::new(self, expression);
            self.root_node.set(Some(node));
        }
    }

    pub fn add_to_generated_code(&self, code: &mut CodeGenerator) {
        let version = &self.context.version;
        match &self.production.versioning {
            ProductionVersioning::Unversioned(_) => {
                let first_version = self.context.grammar.versions.first().unwrap();
                if version != first_version {
                    return;
                }
            }
            ProductionVersioning::Versioned(versions) => {
                if !versions.contains_key(version) {
                    return;
                }
            }
        }

        let name = self.production.name.clone();
        let comment = self.production.generate_ebnf(self.context.grammar);

        match self.production.kind {
            ProductionKind::Rule => {
                code.add_rule_kind(self.production.name.clone());
                let parser = self.root_node.get().unwrap().to_parser_code(false, code);
                code.add_parser(name, version, comment, parser);
            }

            ProductionKind::Trivia => {
                code.add_rule_kind(self.production.name.clone());
                let parser = self.root_node.get().unwrap().to_parser_code(true, code);
                code.add_parser(name, version, comment, parser);
            }

            ProductionKind::Token => {
                if self.first_set().includes_epsilon {
                    unreachable!("Validation should have discovered that token production {} can generate empty results", name);
                }
                code.add_token_kind(self.production.name.clone());

                let root_node = self.root_node.get().unwrap();

                let scanner = root_node.to_scanner_code(code);
                code.add_scanner(name, version, comment, scanner);
            }
        }
    }

    pub fn expression(&self) -> ExpressionRef {
        return match &self.production.versioning {
            ProductionVersioning::Unversioned(expression) => expression.clone(),
            ProductionVersioning::Versioned(versions) => {
                let version = &self.context.version;
                versions
                    .iter()
                    .filter(|(v, _)| *v <= version)
                    .last()
                    .map(|(_, e)| e.clone())
                    .expect(&format!(
                        "Production {} has no content for version {}",
                        self.production.name, version
                    ))
            }
        };
    }

    pub(crate) fn to_precedence_rule_operator(
        &'context self,
        parent_tree: &'context CombinatorTree<'context>,
    ) -> Option<PrecedenceRuleOperator> {
        self.ensure_tree_is_built();

        if let Some(CombinatorNode::Sequence { elements, .. }) = self.root_node.get() {
            let last_element_index = elements.len() - 1;

            let left = if let CombinatorNode::Reference { tree } = elements[0] {
                tree.production.name == parent_tree.production.name
            } else {
                false
            };

            let right = if let CombinatorNode::Reference { tree, .. } = elements[last_element_index]
            {
                tree.production.name == parent_tree.production.name
            } else {
                false
            };

            let (operator, model) = match (&left, &right) {
                (false, false) => return None,
                (false, true) => (
                    elements[..last_element_index]
                        .into_iter()
                        .map(|v| *v)
                        .collect::<Vec<_>>(),
                    OperatorModel::UnaryPrefix,
                ),
                (true, false) => (
                    elements[1..].into_iter().map(|v| *v).collect::<Vec<_>>(),
                    OperatorModel::UnarySuffix,
                ),
                (true, true) => (
                    elements[1..last_element_index]
                        .into_iter()
                        .map(|v| *v)
                        .collect::<Vec<_>>(),
                    if self.expression().config.associativity
                        == Some(ExpressionAssociativity::Right)
                    {
                        OperatorModel::BinaryRightAssociative
                    } else {
                        OperatorModel::BinaryLeftAssociative
                    },
                ),
            };

            let operator = if operator.len() == 1 {
                operator[0]
            } else {
                self.context.alloc_node(CombinatorNode::Sequence {
                    name: None,
                    elements: operator,
                })
            };

            Some(PrecedenceRuleOperator {
                name: self.production.name.clone(),
                model,
                operator: operator,
            })
        } else {
            return None;
        }
    }
}
