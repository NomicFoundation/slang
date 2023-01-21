use codegen_schema::types::productions::{ExpressionParser, ExpressionRef, ProductionKind};
use patricia_tree::{node::Node, PatriciaMap};
use proc_macro2::TokenStream;
use quote::quote;

use super::{combinator_tree::CombinatorTree, naming};

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaMap<TreeEntry>);

#[derive(Clone, Debug)]
pub struct TreeEntry {
    pub name: Option<String>,
    pub payload: TokenStream,
}

impl TerminalTrie {
    pub fn new() -> Self {
        Self(PatriciaMap::new())
    }

    pub fn from_expression(
        tree: &CombinatorTree,
        expression: &ExpressionRef,
    ) -> Option<TerminalTrie> {
        let mut trie = Self::new();
        if trie.collect_terminals(
            tree,
            expression,
            tree.production.kind != ProductionKind::Token,
        ) && trie.0.len() > 0
        {
            if tree.production.kind == ProductionKind::Token
                && trie.0.iter().any(|(_, entry)| entry.name.is_some())
            {
                trie = Self(PatriciaMap::new());
                trie.collect_terminals(tree, expression, true);
            }
            Some(trie)
        } else {
            None
        }
    }

    fn collect_terminals(
        &mut self,
        tree: &CombinatorTree,
        expression: &ExpressionRef,
        force_all_entries_to_be_named: bool,
    ) -> bool {
        match &expression.parser {
            ExpressionParser::Choice(exprs) => exprs.iter().fold(true, |accum, e| {
                accum && self.collect_terminals(tree, e, force_all_entries_to_be_named)
            }),

            ExpressionParser::Terminal(string) => {
                self.0.insert(
                    string.clone(),
                    TreeEntry {
                        name: expression.config.name.clone().or_else(|| {
                            if tree.production.kind == ProductionKind::Token
                                && !force_all_entries_to_be_named
                            {
                                None
                            } else {
                                Some(naming::name_of_terminal_string(&string))
                            }
                        }),
                        payload: quote!(()),
                    },
                );
                true
            }

            ExpressionParser::Reference(name) => {
                tree.production.kind == ProductionKind::Token
                    && self.collect_terminals(
                        tree,
                        &tree.context.get_tree_by_name(name).expression(),
                        force_all_entries_to_be_named,
                    )
            }

            _ => false,
        }
    }

    pub(super) fn has_named_structure(&self) -> bool {
        self.0.iter().next().unwrap().1.name.is_some()
    }

    pub(super) fn set_payloads(&mut self, payload: TokenStream) {
        for (_, entry) in self.0.iter_mut() {
            entry.payload = payload.clone();
        }
    }

    pub(super) fn merged_with(&self, curr: TerminalTrie) -> TerminalTrie {
        let mut new_set = self.0.clone();
        new_set.extend(curr.0.into_iter());
        Self(new_set)
    }

    pub(super) fn generate<NV, LV>(&self, node_visitor: &NV, leaf_visitor: &LV) -> TokenStream
    where
        NV: Fn(&str, &[TokenStream]) -> TokenStream,
        LV: Fn(&TreeEntry, &str) -> TokenStream,
    {
        if self.0.len() == 0 {
            unreachable!("A TerminalTrie with no entries should never have beeen constructed")
        }

        fn generate_entry<NV, LV>(
            node: Option<&Node<TreeEntry>>,
            node_visitor: &NV,
            leaf_visitor: &LV,
        ) -> Vec<TokenStream>
        where
            NV: Fn(&str, &[TokenStream]) -> TokenStream,
            LV: Fn(&TreeEntry, &str) -> TokenStream,
        {
            let mut result = vec![];
            let mut n = node;
            while let Some(node) = n {
                let label = String::from_utf8_lossy(node.label());
                let mut children = generate_entry(node.child(), node_visitor, leaf_visitor);
                if let Some(entry) = node.value() {
                    if children.is_empty() {
                        result.push(leaf_visitor(&entry, label.to_string().as_str()));
                    } else {
                        children.push(leaf_visitor(&entry, ""));
                    }
                }

                if !children.is_empty() {
                    result.push(node_visitor(&label, &children));
                }
                n = node.sibling()
            }

            result
        }

        let mut result = generate_entry(self.0.as_ref().child(), node_visitor, leaf_visitor);
        if result.len() == 1 {
            result.pop().unwrap()
        } else {
            quote!(choice!( #(#result),* ))
        }
    }
}
