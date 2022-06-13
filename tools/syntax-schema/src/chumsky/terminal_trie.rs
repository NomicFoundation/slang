use patricia_tree::{node::Node, PatriciaSet};
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::slang_name::SlangName;

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaSet);

impl TerminalTrie {
    pub fn slang_name(&self) -> Option<SlangName> {
        if self.0.len() == 1 {
            let node = self.0.as_ref().child().unwrap();
            let name = String::from_utf8_lossy(node.label()).to_string();
            Some(SlangName::from_terminal(&name))
        } else {
            None
        }
    }

    pub fn common_terminal_length(&self) -> Option<usize> {
        let mut iter = self
            .0
            .iter()
            .map(|x| String::from_utf8_lossy(&x).into_owned());
        let len = iter.next().unwrap().chars().count();
        if iter.all(|x| x.chars().count() == len) {
            Some(len)
        } else {
            None
        }
    }

    pub fn to_parser_combinator_code(&self) -> TokenStream {
        let common_size = self.common_terminal_length();

        fn generate_from_trie(
            node: Option<&Node<()>>,
            length: usize,
            ignore_all: bool,
        ) -> Vec<TokenStream> {
            let mut result = vec![];
            let mut n = node;
            while let Some(node) = n {
                let label = String::from_utf8_lossy(node.label());
                let new_length = length + label.chars().count();
                let mut children = generate_from_trie(node.child(), new_length, ignore_all);
                if node.child().is_some() && node.value().is_some() {
                    if ignore_all {
                        children.push(quote!(empty()));
                    } else {
                        children.push(quote!( empty().map(|_| #new_length) ));
                    }
                }
                if children.is_empty() {
                    if ignore_all {
                        result.push(quote!( terminal(#label).ignored() ))
                    } else {
                        result.push(quote!( terminal(#label).map(|_| #new_length) ))
                    }
                } else if children.len() == 1 {
                    let child = &children[0];
                    result.push(quote!( terminal(#label).ignore_then(#child) ))
                } else {
                    result.push(quote!( terminal(#label).ignore_then(choice((
                                        #(#children),*
                                    ))) ))
                }
                n = node.sibling()
            }
            result
        }

        let mut choices = generate_from_trie(self.0.as_ref().child(), 0, common_size.is_some());

        let code = if choices.len() == 1 {
            choices.pop().unwrap()
        } else {
            quote!( choice::<_, ErrorType>((#(#choices),*)) )
        };

        if let Some(size) = common_size {
            quote!( #code.map(|_| FixedTerminal::<#size>()) )
        } else {
            code
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        other.0.into_iter().for_each(|k| {
            self.0.insert(k);
        });
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
                accum.insert(string.clone());
                true
            }
            EBNF::Choice(exprs) => exprs.iter().all(|e| e.collect_terminals(grammar, accum)),
            EBNF::Reference(name) => grammar
                .get_production(name)
                .map(|p| p.expression_to_generate().collect_terminals(grammar, accum))
                .unwrap_or(false),
            EBNF::Sequence(_) => false, // TODO: special case this
            EBNF::End | EBNF::Repeat(_) | EBNF::Not(_) | EBNF::Difference(_) | EBNF::Range(_) => {
                false
            }
        }
    }
}
