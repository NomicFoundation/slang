use codegen_schema::ProductionKind;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
    character_filter::CharacterFilter,
    code_fragments::CodeFragments,
    combinator_forest::CombinatorForest,
    combinator_node::{CombinatorNode, OperatorModel},
    combinator_tree::CombinatorTree,
    naming,
    terminal_trie::TerminalTrie,
};

pub enum ASTType {
    Rule(String),                           // field name and type name
    Token(String),                          // field name, type is (kinds::Token, Range<usize>)
    Terminal(Option<String>),               // field name, type is Range<usize>
    TerminalChoice(Option<String>),         // field name, type is (kinds::Token, Range<usize>)
    Choice(Option<String>, Vec<ASTType>),   // field name and type name
    Sequence(Option<String>, Vec<ASTType>), // field name and type name
    Repeat(Box<ASTType>),                   // field name is plural of the wrapped type field name
    Optional(Box<ASTType>),                 // field name is the wrapped type field name
}

impl ASTType {
    pub fn field_name(&self) -> Option<String> {
        match self {
            ASTType::Rule(name) | ASTType::Token(name) => Some(name.clone()),
            ASTType::Terminal(name)
            | ASTType::TerminalChoice(name)
            | ASTType::Choice(name, _)
            | ASTType::Sequence(name, _) => name.clone(),
            ASTType::Repeat(ty) => ty.field_name().map(|s| naming::pluralize(&s)),
            ASTType::Optional(ty) => ty.field_name(),
        }
    }
}

pub struct ParserCode {
    pub parser: TokenStream,
    pub ast_type: ASTType,
}

impl TerminalTrie {
    pub fn to_parser_code(
        &self,
        is_trivia: bool,
        name: Option<&String>,
        code: &mut CodeFragments,
    ) -> ParserCode {
        ParserCode {
            parser: self.to_code(code, if is_trivia { "trivia_" } else { "" }),
            ast_type: if self.0.len() == 1 {
                ASTType::Terminal(None) // not surfaced because it is a constant
            } else {
                ASTType::TerminalChoice(name.cloned())
            },
        }
    }
}

impl CharacterFilter {
    pub fn to_parser_code(
        &self,
        is_trivia: bool,
        name: Option<&String>,
        code: &mut CodeFragments,
    ) -> ParserCode {
        ParserCode {
            parser: self.to_code(name, code, if is_trivia { "trivia_" } else { "" }),
            ast_type: ASTType::Terminal(name.cloned()), // not a constant
        }
    }
}

impl CombinatorNode {
    pub fn to_parser_code(
        &self,
        forest: &CombinatorForest,
        tree: &CombinatorTree,
        code: &mut CodeFragments,
    ) -> ParserCode {
        fn create_kind(name: &Option<String>, code: &mut CodeFragments) -> Option<Ident> {
            name.as_ref().map(|name| code.add_rule_kind(name.clone()))
        }

        let is_trivia = tree.production.kind == ProductionKind::Trivia;
        let macro_prefix = if is_trivia { "trivia_" } else { "" };
        let terminal_macro = format_ident!("{}terminal", macro_prefix);
        let token_macro = format_ident!("{}terminal", macro_prefix);

        match self {
            // -----------------------------------------------------------------------------------------------
            // Simple References
            //
            Self::Reference { production } => {
                let production_parser_name = naming::to_parser_name_ident(&production.name);
                let tipe = format_ident!("{}", production.name.to_pascal_case());
                match production.kind {
                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule
                        if is_trivia =>
                    {
                        unreachable!(
                            "Trivia productions can only reference trivia or token productions"
                        )
                    }

                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule
                    | ProductionKind::Trivia => ParserCode {
                        parser: quote!(rule!(#production_parser_name)),
                        ast_type: ASTType::Rule(production.name.clone()),
                    },
                    ProductionKind::Token => ParserCode {
                        parser: quote!(#token_macro!(#production_parser_name)),
                        ast_type: ASTType::Token(production.name.clone()),
                    },
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Sequence and Choice
            //
            Self::Sequence { elements, name } => {
                let (parsers, tipes): (Vec<TokenStream>, Vec<ASTType>) = elements
                    .iter()
                    .map(|element| {
                        let pc = element.to_parser_code(forest, tree, code);
                        (pc.parser, pc.ast_type)
                    })
                    .unzip();

                if let Some(kind) = create_kind(name, code) {
                    let names = elements
                        .iter()
                        .filter_map(|element| {
                            element
                                .name()
                                .map(|name| naming::to_field_name_ident(&name))
                        })
                        .collect::<Vec<_>>();

                    if names.len() == elements.len() {
                        // ≡ all elements are named
                        ParserCode {
                            parser: quote!(seq!(#kind,  #( #names: #parsers ),* )),
                            ast_type: quote!(#kind),
                        }
                    } else {
                        ParserCode {
                            parser: quote!(seq!(#kind, #( #parsers ),* )),
                            ast_type: quote!(#kind),
                        }
                    }
                } else {
                    ParserCode {
                        parser: quote!(seq!( #( #parsers ),* )),
                        ast_type: quote!( ( #( #tipes ),* ) ),
                    }
                }
            }

            Self::Choice { elements, name } => {
                let (parsers, tipes): (Vec<TokenStream>, Vec<TokenStream>) = elements
                    .iter()
                    .map(|element| {
                        let pc = element.to_parser_code(forest, tree, code);
                        (pc.parser, pc.ast_type)
                    })
                    .unzip();

                let has_homogeneous_types = tipes
                    .iter()
                    .all(|tipe| tipe.to_string() == tipes[0].to_string());

                if let Some(kind) = create_kind(name, code) {
                    let names = elements
                        .iter()
                        .filter_map(|element| {
                            element
                                .name()
                                .map(|name| naming::to_enum_entry_ident(&name))
                        })
                        .collect::<Vec<_>>();

                    if names.len() == elements.len() {
                        // ≡ all elements are named
                        ParserCode {
                            parser: quote!(choice!!(#kind, #( #names: #parsers ),* )),
                            ast_type: quote!(#kind),
                        }
                    } else {
                        ParserCode {
                            parser: quote!(choice!(#kind, #( #parsers ),* )),
                            ast_type: quote!(#kind),
                        }
                    }
                } else if has_homogeneous_types {
                    // If the choice elements all return the same type, then
                    // the choice can be elided. Common with token choices.

                    if let Some(kind) = create_kind(name, code) {
                        ParserCode {
                            parser: quote!(choice!(#kind, #( #parsers ),* )),
                            ast_type: tipes[0].clone(),
                        }
                    } else {
                        ParserCode {
                            parser: quote!(choice!( #( #parsers ),* )),
                            ast_type: tipes[0].clone(),
                        }
                    }
                } else {
                    ParserCode {
                        parser: quote!(choice!( #( #parsers ),* )),
                        ast_type: quote!( ( #( Option<#tipes> ),* ) ),
                    }
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Numeric Quantifiers
            //
            Self::Optional { expr, .. } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                ParserCode {
                    parser: quote!(optional!(#expr)),
                    ast_type: ASTType::Optional(Box::new(expr_type)),
                }
            }

            Self::ZeroOrMore { expr, name } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    ParserCode {
                        parser: quote!(zero_or_more!(#kind, #expr)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                } else {
                    ParserCode {
                        parser: quote!(zero_or_more!(#expr)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                }
            }

            Self::OneOrMore { expr, name } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    ParserCode {
                        parser: quote!(one_or_more!(#kind, #expr)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                } else {
                    ParserCode {
                        parser: quote!(one_or_more!(#expr)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    ParserCode {
                        parser: quote!(repeated!(#kind, #expr, #min, #max)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                } else {
                    ParserCode {
                        parser: quote!(repeated!(#expr, #min, #max)),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Stereotypes
            //
            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    ParserCode {
                        parser: quote!(
                            delimited_by!(#kind, #terminal_macro!(#open_kind, #open), #expr, #terminal_macro!(#close_kind, #close))
                        ),
                        ast_type: expr_type,
                    }
                } else {
                    ParserCode {
                        parser: quote!(
                            delimited_by!(#terminal_macro!(#open_kind, #open), #expr, #terminal_macro!(#close_kind, #close))
                        ),
                        ast_type: expr_type,
                    }
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_type,
                } = expr.to_parser_code(forest, tree, code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    ParserCode {
                        parser: quote!(
                            separated_by!(#kind, #expr, #terminal_macro!(#separator_kind, #separator))
                        ),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                } else {
                    ParserCode {
                        parser: quote!(
                            separated_by!(#expr, #terminal_macro!(#separator_kind, #separator))
                        ),
                        ast_type: ASTType::Repeat(Box::new(expr_type)),
                    }
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Expressions
            //
            Self::Expression { members, name } => {
                if is_trivia {
                    unreachable!("Expressions are not allowed in trivia")
                }
                let first_parser_name = naming::to_parser_name_ident(&members[0].name);
                let type_name = format_ident!("{}", name.to_pascal_case());
                ParserCode {
                    parser: quote!(rule!(#first_parser_name)),
                    ast_type: quote!(#type_name),
                }
            }

            Self::ExpressionMember {
                next_sibling,
                operator,
                operator_model,
                name,
                parent,
            } => {
                if is_trivia {
                    unreachable!("ExpressionMembers are not allowed in trivia")
                }
                let parent_type_name = format_ident!("{}", parent.name.to_snake_case());
                let kind = code.add_rule_kind(name.clone());
                let ParserCode {
                    parser: operator,
                    ast_type: operator_type,
                } = operator.to_parser_code(forest, tree, code);
                let next_sibling = next_sibling
                    .clone()
                    .map(|next| naming::to_parser_name_ident(&next.name));

                match operator_model {
                    OperatorModel::None => match next_sibling {
                        Some(next_sibling) => ParserCode {
                            parser: quote!( choice((#operator, #next_sibling.clone())) ),
                            ast_type: quote!(#parent_type_name),
                        },
                        None => ParserCode {
                            parser: operator,
                            ast_type: operator_type,
                        },
                    },

                    OperatorModel::BinaryLeftAssociative => {
                        let next_sibling = next_sibling
                            .expect("Cannot have binary operator as last expression member");
                        ParserCode {
                            parser: quote!(
                                left_associative_binary_expression!(#kind, #next_sibling, #operator)
                            ),
                            ast_type: quote!(#parent_type_name),
                        }
                    }

                    OperatorModel::BinaryRightAssociative => {
                        let next_sibling = next_sibling
                            .expect("Cannot have binary operator as last expression member");
                        ParserCode {
                            parser: quote!(
                                right_associative_binary_expression!(#kind, #next_sibling, #operator)
                            ),
                            ast_type: quote!(#parent_type_name),
                        }
                    }

                    OperatorModel::UnaryPrefix => {
                        let next_sibling = next_sibling
                            .expect("Cannot have unary operator as last expression member");
                        ParserCode {
                            parser: quote!(
                                unary_prefix_expression!(#kind, #next_sibling, #operator)
                            ),
                            ast_type: quote!(#parent_type_name),
                        }
                    }

                    OperatorModel::UnarySuffix => {
                        let next_sibling = next_sibling
                            .expect("Cannot have unary operator as last expression member");
                        ParserCode {
                            parser: quote!(
                                unary_suffix_expression!(#kind, #next_sibling, #operator)
                            ),
                            ast_type: quote!(#parent_type_name),
                        }
                    }
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Terminals and Utilities
            //
            Self::CharacterFilter { filter, name } => {
                filter.to_parser_code(is_trivia, name.as_ref(), code)
            }

            Self::TerminalTrie { trie, name } => {
                trie.to_parser_code(is_trivia, name.as_ref(), code)
            }

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let ParserCode {
                    parser: minuend,
                    ast_type: minuend_type,
                } = minuend.to_parser_code(forest, tree, code);
                let ParserCode {
                    parser: subtrahend, ..
                } = subtrahend.to_parser_code(forest, tree, code);
                ParserCode {
                    parser: quote!(difference!(#minuend, #subtrahend)),
                    ast_type: minuend_type,
                }
            }

            Self::Lookahead { expr, lookahead } => {
                let ParserCode {
                    parser: expr,
                    ast_type: expr_tipe,
                } = expr.to_parser_code(forest, tree, code);
                let ParserCode {
                    parser: lookahead, ..
                } = lookahead.to_parser_code(forest, tree, code);
                ParserCode {
                    parser: quote!( #expr.then_ignore(#lookahead.rewind()) ),
                    ast_type: expr_tipe,
                }
            }
        }
    }
}
