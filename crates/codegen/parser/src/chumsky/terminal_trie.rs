use patricia_tree::{node::Node, PatriciaMap};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use codegen_schema::*;

use super::{code_generator::CodeGenerator, combinator_tree::CombinatorTree, naming};

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaMap<String>);

impl TerminalTrie {
    pub fn new(
        tree: &CombinatorTree,
        expression: &ExpressionRef,
        inline_references: bool,
    ) -> Option<TerminalTrie> {
        let mut trie = Self(PatriciaMap::new());
        if trie.collect_terminals(tree, expression, inline_references) && trie.0.len() > 0 {
            Some(trie)
        } else {
            None
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

    fn collect_terminals(
        &mut self,
        tree: &CombinatorTree,
        expression: &ExpressionRef,
        inline_references: bool,
    ) -> bool {
        match &expression.ebnf {
            EBNF::Choice(exprs) => exprs.iter().fold(true, |accum, e| {
                self.collect_terminals(tree, e, inline_references) && accum
            }),

            EBNF::Terminal(string) => {
                self.0.insert(string.clone(), string.clone());
                true
            }

            EBNF::Reference(name) => {
                inline_references
                    && self.collect_terminals(
                        tree,
                        &tree.context.get_tree_by_name(name).expression(),
                        inline_references,
                    )
            }

            _ => false,
        }
    }

    pub(super) fn merged_with(&self, curr: TerminalTrie) -> TerminalTrie {
        let mut new_set = self.0.clone();
        new_set.extend(curr.0.into_iter());
        Self(new_set)
    }

    pub(super) fn to_code(
        &self,
        kind: Option<Ident>,
        code: &mut CodeGenerator,
        macro_prefix: &str,
    ) -> TokenStream {
        if self.0.len() == 1 {
            let label = self.0.keys().next().unwrap();
            let string = String::from_utf8_lossy(&label);
            let kind = code.add_terminal_kind(string.to_string());
            let macro_name = format_ident!("{}terminal", macro_prefix);
            quote!(#macro_name!(#kind, #string))
        } else {
            fn generate_from_trie(
                node: Option<&Node<String>>,
                code: &mut CodeGenerator,
            ) -> Vec<TokenStream> {
                let mut result = vec![];
                let mut n = node;
                while let Some(node) = n {
                    let label = String::from_utf8_lossy(node.label());
                    let mut children = generate_from_trie(node.child(), code);
                    if let Some(string) = node.value() {
                        let kind = code.add_terminal_kind(string.clone());
                        if children.is_empty() {
                            result.push(quote!(trieleaf!(#kind, #label)))
                        } else {
                            children.push(quote!(trieleaf!(#kind)))
                        }
                    }

                    if !children.is_empty() {
                        if children.len() == 1 {
                            unreachable!()
                        } else {
                            result.push(quote!(trieprefix!(#label, [ #(#children),* ])))
                        }
                    }
                    n = node.sibling()
                }

                result
            }

            if self.0.len() == 1 {
                unreachable!()
            }

            let lexers = generate_from_trie(self.0.as_ref().child(), code);
            let macro_name = format_ident!("{}trie", macro_prefix);
            if let Some(kind) = kind {
                quote!(#macro_name!(#kind, #(#lexers),*))
            } else {
                quote!(#macro_name!(#(#lexers),*))
            }
        }
    }
}
