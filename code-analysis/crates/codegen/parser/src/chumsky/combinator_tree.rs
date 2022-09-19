use std::{cell::Cell, collections::BTreeSet, rc::Rc};

use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use codegen_schema::*;

use super::{
    combinator_forest::CombinatorForest,
    combinator_node::{CodeForNode, CombinatorNode, CombinatorNodeRef, OperatorModel},
    naming, ProductionChumskyExtensions,
};

#[derive(Clone, Debug)]
pub struct CombinatorTree {
    pub name: String,
    pub production: ProductionRef,
    pub root_node: CombinatorNodeRef,
}

pub type CombinatorTreeRef = Rc<CombinatorTree>;

pub struct CodeForTree {
    pub cst_token_part_kinds: BTreeSet<Ident>,
    pub cst_token_kinds: BTreeSet<Ident>,
    pub cst_rule_kinds: BTreeSet<Ident>,

    pub cst_parser_field_decl_fragment: TokenStream,

    pub cst_parser_impl_predecl_fragment: TokenStream,
    pub cst_parser_impl_fragment: TokenStream,
    pub cst_parser_impl_field_init_fragment: TokenStream,

    pub ast_fragment: TokenStream,
    pub ast_impl_fragment: TokenStream,

    pub ast_parser_field_decl_fragment: TokenStream,

    pub ast_parser_impl_predecl_fragment: TokenStream,
    pub ast_parser_impl_fragment: TokenStream,
    pub ast_parser_impl_field_init_fragment: TokenStream,
}

impl CombinatorTree {
    pub fn from_production(grammar: &GrammarRef, production: &ProductionRef) -> CombinatorTreeRef {
        Rc::new(CombinatorTree {
            name: production.name.to_snake_case(),
            root_node: CombinatorNode::from_expression(
                grammar,
                production,
                &production.expression_to_generate(),
                Some(production.name.to_snake_case()),
                &Cell::new(0),
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
                let left =
                    if let CombinatorNode::Reference { production, .. } = elements[0].as_ref() {
                        production.name == this.name()
                    } else {
                        false
                    };

                let right = if let CombinatorNode::Reference { production, .. } =
                    elements[elements.len() - 1].as_ref()
                {
                    production.name == this.name()
                } else {
                    false
                };

                let (operator, model) = match (&left, &right) {
                    (false, false) => (&elements[..], OperatorModel::None),
                    (false, true) => (&elements[..elements.len() - 1], OperatorModel::UnaryPrefix),
                    (true, false) => (&elements[1..], OperatorModel::UnarySuffix),
                    (true, true) => (
                        &elements[1..elements.len() - 1],
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
                    {
                        let name = format!("anonexpfrag_{}", elements.len() + 1);
                        let elements = operator.into();
                        CombinatorNode::new_sequence(name, elements)
                    }
                };

                {
                    let name = this.name();
                    CombinatorNode::new_expression_member(
                        name,
                        parent,
                        next_sibling,
                        operator,
                        model,
                    )
                }
            } else {
                {
                    let name = this.name();
                    let operator = this.clone();
                    let operator_model = OperatorModel::None;
                    CombinatorNode::new_expression_member(
                        name,
                        parent,
                        next_sibling,
                        operator,
                        operator_model,
                    )
                }
            }
        };
    }

    pub fn referenced_identifiers(&self) -> BTreeSet<String> {
        let mut accum = BTreeSet::new();
        self.root_node.collect_identifiers(&mut accum);
        if !self.production.is_token() {
            accum.insert("LeadingTrivia".to_owned());
            accum.insert("TrailingTrivia".to_owned());
        }
        accum
    }

    pub fn to_generated_code(&self, forest: &CombinatorForest, is_recursive: bool) -> CodeForTree {
        let CodeForNode {
            cst_token_part_kinds,
            cst_token_kinds,
            cst_rule_kinds,
            cst_parser_impl_fragment,
            mut ast_fragment,
            mut ast_impl_fragment,
            ast_parser_type,
            ast_parser_impl_fragment,
        } = self.root_node.to_generated_code(forest, self);

        let module_name = naming::to_module_name_ident(&self.name);
        let type_name = naming::to_type_name_ident(&self.name);
        let parser_name = naming::to_parser_name_ident(&self.name);
        let field_name = naming::to_field_name_ident(&self.name);

        if self.production.is_token()
            && self.production.name != "LeadingTrivia"
            && self.production.name != "TrailingTrivia"
        {
            let parser_type = ast_parser_type.clone();
            if self.root_node.has_default(forest) {
                ast_fragment.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
                    pub struct WithTrivia {
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub leading_trivia: LeadingTrivia,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub terminal: #parser_type,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub trailing_trivia: TrailingTrivia,
                    }
                ));
                ast_impl_fragment.push(quote!(
                    impl DefaultTest for #module_name::WithTrivia {
                        fn is_default(&self) -> bool {
                            self.leading_trivia.is_default() && self.terminal.is_default() && self.trailing_trivia.is_default()
                        }
                    }
                ))
            } else {
                ast_fragment.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct WithTrivia {
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub leading_trivia: LeadingTrivia,
                        pub terminal: #parser_type,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub trailing_trivia: TrailingTrivia,
                    }
                ));
            }
        }

        CodeForTree {
            cst_token_part_kinds,
            cst_token_kinds,
            cst_rule_kinds,

            cst_parser_field_decl_fragment: quote!( pub #field_name: ParserType<NodeRef> ),

            cst_parser_impl_field_init_fragment: if is_recursive {
                quote!( #field_name: #parser_name.then_ignore(end()).boxed() )
            } else {
                quote!( #field_name: #parser_name.then_ignore(end()).boxed() )
            },
            cst_parser_impl_predecl_fragment: if is_recursive {
                quote!( let mut #parser_name = Recursive::declare(); )
            } else {
                quote!()
            },
            cst_parser_impl_fragment: if is_recursive {
                quote!( #parser_name.define(#cst_parser_impl_fragment.boxed()); )
            } else {
                quote!( let #parser_name = #cst_parser_impl_fragment.boxed(); )
            },

            ast_fragment: {
                let module = if ast_fragment.is_empty() {
                    quote!()
                } else {
                    quote!(
                        pub mod #module_name {
                        #[allow(unused_imports)]
                        use super::*;
                        #(#ast_fragment)*
                    })
                };
                quote!(
                    pub type #type_name = #ast_parser_type;
                    #module
                )
            },

            ast_impl_fragment: quote!( #(#ast_impl_fragment)* ),

            ast_parser_field_decl_fragment: quote!( pub #field_name: ParserType<#type_name> ),

            ast_parser_impl_field_init_fragment: if is_recursive {
                quote!( #field_name: #parser_name.then_ignore(end()).boxed() )
            } else {
                quote!( #field_name: #parser_name.then_ignore(end()).boxed() )
            },
            ast_parser_impl_predecl_fragment: if is_recursive {
                quote!( let mut #parser_name = Recursive::declare(); )
            } else {
                quote!()
            },
            ast_parser_impl_fragment: if is_recursive {
                quote!( #parser_name.define(#ast_parser_impl_fragment.boxed()); )
            } else {
                quote!( let #parser_name = #ast_parser_impl_fragment.boxed(); )
            },
        }
    }
}
