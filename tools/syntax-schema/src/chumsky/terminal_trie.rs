use patricia_tree::{node::Node, PatriciaSet};
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

pub struct TerminalTrie(PatriciaSet);

impl TerminalTrie {
    pub fn to_parser_expression(&self) -> TokenStream {
        fn generate_from_trie(node: Option<&Node<()>>, length: usize) -> Vec<TokenStream> {
            let mut result = vec![];
            let mut n = node;
            while let Some(node) = n {
                let label = String::from_utf8_lossy(node.label());
                let new_length = length + label.chars().count();
                let mut children = generate_from_trie(node.child(), new_length);
                if node.child().is_some() && node.value().is_some() {
                    children.push(quote!( empty().map(|_| #length) ));
                }
                if children.is_empty() {
                    result.push(quote!( just(#label).map(|_| #new_length) ))
                } else if children.len() == 1 {
                    let child = &children[0];
                    result.push(quote!( just(#label).ignore_then(#child) ))
                } else {
                    result.push(quote!( just(#label).ignore_then(choice((
                                        #(#children),*
                                    ))) ))
                }
                n = node.sibling()
            }
            result
        }

        let mut choices = generate_from_trie(self.0.as_ref().child(), 0);

        if choices.len() == 1 {
            choices.pop().unwrap()
        } else {
            quote!( choice::<_, ErrorType>((#(#choices),*)) )
        }
    }
}

impl Expression {
    pub fn to_terminal_trie(&self, grammar: &Grammar) -> Option<TerminalTrie> {
        let mut accum = PatriciaSet::new();
        if self.collect_terminals(grammar, &mut accum) {
            Some(TerminalTrie(accum))
        } else {
            None
        }
    }

    fn collect_terminals(&self, grammar: &Grammar, accum: &mut PatriciaSet) -> bool {
        match &self.ebnf {
            EBNF::Terminal(string) => {
                // TODO: maybe this test is no longer applicable?
                if self.config.map.is_none() {
                    accum.insert(string.clone());
                    true
                } else {
                    false
                }
            }
            EBNF::Choice(exprs) => exprs.iter().all(|e| e.collect_terminals(grammar, accum)),
            EBNF::Reference(name) => grammar
                .get_production(name)
                .map(|p| p.expression_to_generate().collect_terminals(grammar, accum))
                .unwrap_or(false),
            EBNF::End
            | EBNF::Repeat(_)
            | EBNF::Not(_)
            | EBNF::Sequence(_)
            | EBNF::Difference(_)
            | EBNF::Range(_) => false,
        }
    }
}
