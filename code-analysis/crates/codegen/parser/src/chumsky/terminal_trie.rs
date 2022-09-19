use patricia_tree::{node::Node, PatriciaMap};
use proc_macro2::TokenStream;
use quote::quote;

use codegen_schema::*;

use super::{combinator_node, naming, ProductionChumskyExtensions};

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaMap<String>);

impl TerminalTrie {
    pub fn from_expression(grammar: &Grammar, expression: &ExpressionRef) -> Option<TerminalTrie> {
        let mut trie = Self(PatriciaMap::new());
        trie.collect_terminals(grammar, expression);
        if trie.0.len() > 0 {
            Some(trie)
        } else {
            None
        }
    }

    fn collect_terminals(&mut self, grammar: &Grammar, expression: &ExpressionRef) -> bool {
        match &expression.ebnf {
            EBNF::Choice(exprs) => {
                expression.config.merge == Some(true)
                    && exprs.iter().all(|e| self.collect_terminals(grammar, e))
            }

            EBNF::DelimitedBy(_)
            | EBNF::Difference(_)
            | EBNF::Not(_)
            | EBNF::OneOrMore(_)
            | EBNF::Optional(_)
            | EBNF::Range(_)
            | EBNF::Repeat(_)
            | EBNF::SeparatedBy(_)
            | EBNF::Sequence(_)
            | EBNF::ZeroOrMore(_) => false,

            EBNF::Reference(name) => {
                expression.config.merge == Some(true)
                    && self.collect_terminals(
                        grammar,
                        &grammar.get_production(name).expression_to_generate(),
                    )
            }

            EBNF::Terminal(string) => {
                self.0.insert(string.clone(), string.clone());
                true
            }
        }
    }

    pub fn default_name(&self) -> Option<String> {
        if self.0.len() == 1 {
            let node = self.0.as_ref().child().unwrap();
            let name = String::from_utf8_lossy(node.label()).to_string();
            Some(naming::name_of_terminal_string(&name))
        } else {
            None
        }
    }

    pub fn common_terminal_length(&self) -> Option<usize> {
        let mut iter = self.0.values();
        let len = iter.next().unwrap().chars().count();
        if iter.all(|x| x.chars().count() == len) {
            Some(len)
        } else {
            None
        }
    }

    pub fn to_generated_code(&self, with_trivia: bool) -> combinator_node::CodeForNode {
        let mut result: combinator_node::CodeForNode = Default::default();

        struct Parsers {
            cst: TokenStream,
            ast: TokenStream,
        }

        fn generate_from_trie(
            node: Option<&Node<String>>,
            length: usize,
            is_fixed_size: bool,
        ) -> Vec<Parsers> {
            let mut result = vec![];
            let mut n = node;
            while let Some(node) = n {
                let label = String::from_utf8_lossy(node.label());
                let new_length = length + label.chars().count();
                let mut children = generate_from_trie(node.child(), new_length, is_fixed_size);
                if let Some(terminal) = node.value() {
                    let name = naming::name_of_terminal_string(terminal);
                    let kind = naming::to_kind_ident(&name, &name);
                    if node.child().is_some() {}
                    if children.is_empty() {
                        result.push(if is_fixed_size {
                        Parsers {
                            cst:  quote!( terminal(#label).to(Node::new_token_part(TokenPartKind::#kind, #new_length)) ),
                            ast: quote!( terminal(#label).to(FixedSizeTerminal::<#new_length>()) )
                        }
                    } else {
                        Parsers {
                            cst:  quote!( terminal(#label).to(Node::new_token_part(TokenPartKind::#kind, #new_length)) ),
                            ast: quote!( terminal(#label).to(VariableSizeTerminal(#new_length)) )
                        }
                    })
                    } else {
                        children.push(if is_fixed_size {
                            Parsers {
                            cst:  quote!( empty().to(Node::new_token_part(TokenPartKind::#kind, #new_length)) ),
                                ast: quote!( empty().to(FixedSizeTerminal::<#new_length>()) ),
                            }
                        } else {
                            Parsers {
                            cst:  quote!( empty().to(Node::new_token_part(TokenPartKind::#kind, #new_length)) ),
                                ast: quote!( empty().to(VariableSizeTerminal(#new_length)) ),
                            }
                        })
                    }
                }

                if !children.is_empty() {
                    if children.len() == 1 {
                        let Parsers { cst, ast } = &children[0];
                        result.push(Parsers {
                            cst: quote!( terminal(#label).ignore_then(#cst) ),
                            ast: quote!( terminal(#label).ignore_then(#ast) ),
                        })
                    } else {
                        let (cst_children, ast_children): (Vec<_>, Vec<_>) = children
                            .into_iter()
                            .map(|Parsers { cst, ast }| (cst, ast))
                            .unzip();
                        result.push(Parsers {
                        cst: quote!( terminal(#label).ignore_then(choice(( #(#cst_children),* )))),
                        ast: quote!( terminal(#label).ignore_then(choice(( #(#ast_children),* )))),
                    })
                    }
                }
                n = node.sibling()
            }

            result
        }

        self.0.values().for_each(|terminal| {
            let name = naming::name_of_terminal_string(terminal);
            let kind = naming::to_kind_ident(&name, &name);
            result.cst_token_part_kinds.insert(kind);
        });

        let common_size = self.common_terminal_length();
        let (mut cst_children, mut ast_children): (Vec<_>, Vec<_>) =
            generate_from_trie(self.0.as_ref().child(), 0, common_size.is_some())
                .into_iter()
                .map(|Parsers { cst, ast }| (cst, ast))
                .unzip();

        let cst_parser = if cst_children.len() == 1 {
            cst_children.pop().unwrap()
        } else {
            quote!( choice::<_, ErrorType>((#(#cst_children),*)) )
        };

        let ast_parser = if ast_children.len() == 1 {
            ast_children.pop().unwrap()
        } else {
            quote!( choice::<_, ErrorType>((#(#ast_children),*)) )
        };

        let (cst_parser, ast_parser, ast_parser_type) = match (common_size, with_trivia) {
            (None, false) => (cst_parser, ast_parser, quote!(VariableSizeTerminal)),

            (Some(size), false) => (cst_parser, ast_parser, quote!( FixedSizeTerminal<#size> )),

            (None, true) => (
                quote!( leading_trivia_parser.clone().then(#cst_parser).then(trailing_trivia_parser.clone()).map(Node::new_with_trivia) ),
                quote!(
                    leading_trivia_parser.clone().then(#ast_parser).then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)|
                        VariableSizeTerminalWithTrivia { leading_trivia, terminal, trailing_trivia })
                ),
                quote!(VariableSizeTerminalWithTrivia),
            ),

            (Some(size), true) => (
                quote!( leading_trivia_parser.clone().then(#cst_parser).then(trailing_trivia_parser.clone()).map(Node::new_with_trivia) ),
                quote!(
                    leading_trivia_parser.clone().then(#ast_parser).then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia { leading_trivia, terminal, trailing_trivia })
                ),
                quote!( FixedSizeTerminalWithTrivia<#size> ),
            ),
        };
        result.cst_parser_impl_fragment = cst_parser;
        result.ast_parser_impl_fragment = ast_parser;
        result.ast_parser_type = ast_parser_type;

        result
    }

    pub fn merge_with(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}
