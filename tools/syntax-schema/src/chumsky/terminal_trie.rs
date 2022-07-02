use patricia_tree::{node::Node, PatriciaSet};
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::{combinator_tree::GeneratedCode, name::Name};

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaSet);

impl TerminalTrie {
    pub fn slang_name(&self) -> Name {
        if self.0.len() == 1 {
            let node = self.0.as_ref().child().unwrap();
            let name = String::from_utf8_lossy(node.label()).to_string();
            Name::from_terminal(&name)
        } else {
            Name::anonymous()
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

    pub fn to_generated_code(&self, with_noise: bool) -> GeneratedCode {
        let mut result: GeneratedCode = Default::default();

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

        let common_size = self.common_terminal_length();
        let mut choices = generate_from_trie(self.0.as_ref().child(), 0, common_size.is_some());

        let parser = if choices.len() == 1 {
            choices.pop().unwrap()
        } else {
            quote!( choice::<_, ErrorType>((#(#choices),*)) )
        };

        let (parser, parser_type) = match (common_size, with_noise) {
            (None, false) => (
                quote!( #parser.map(VariableSizeTerminal) ),
                quote!(VariableSizeTerminal),
            ),
            (Some(size), false) => (
                quote!( #parser.map(|_| FixedSizeTerminal::<#size>()) ),
                quote!( FixedSizeTerminal<#size> ),
            ),
            (None, true) => (
                quote!(
                    ignore_parser.clone().then(#parser).then(ignore_parser.clone())
                    .map(|((leading, content), trailing)|
                        VariableSizeTerminalWithNoise { leading, content: VariableSizeTerminal(content), trailing })
                ),
                quote!(VariableSizeTerminalWithNoise),
            ),
            (Some(size), true) => (
                quote!(
                    ignore_parser.clone().then(#parser.map(|_| FixedSizeTerminal::<#size>())).then(ignore_parser.clone())
                    .map(|((leading, content), trailing)| FixedSizeTerminalWithNoise { leading, content, trailing })
                ),
                quote!( FixedSizeTerminalWithNoise<#size> ),
            ),
        };
        result.parser = parser;
        result.parser_type = parser_type;

        result
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
                .expression_to_generate()
                .collect_terminals(grammar, accum),
            EBNF::Sequence(_) => false, // TODO: special case this i.e. 'multiply' the sequence elements?
            EBNF::End | EBNF::Repeat(_) | EBNF::Not(_) | EBNF::Difference(_) | EBNF::Range(_) => {
                false
            }
        }
    }
}
