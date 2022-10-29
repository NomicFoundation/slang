use patricia_tree::{node::Node, PatriciaMap};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use codegen_schema::*;
use semver::Version;

use super::{code_fragments::CodeFragments, naming, production::ProductionChumskyExtensions};

#[derive(Clone, Debug)]
pub struct TerminalTrie(PatriciaMap<String>);

impl TerminalTrie {
    pub fn from_expression(
        grammar: &Grammar,
        version: &Version,
        expression: &ExpressionRef,
    ) -> Option<TerminalTrie> {
        let mut trie = Self(PatriciaMap::new());
        trie.collect_terminals(grammar, version, expression);
        if trie.0.len() > 0 {
            Some(trie)
        } else {
            None
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        self.0.extend(other.0);
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
        grammar: &Grammar,
        version: &Version,
        expression: &ExpressionRef,
    ) -> bool {
        match &expression.ebnf {
            EBNF::Choice(exprs) => exprs
                .iter()
                .all(|e| self.collect_terminals(grammar, version, e)),

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

            EBNF::Reference(name) => self.collect_terminals(
                grammar,
                version,
                &grammar.get_production(name).expression_for_version(version),
            ),

            EBNF::Terminal(string) => {
                self.0.insert(string.clone(), string.clone());
                true
            }
        }
    }

    pub(super) fn to_code(&self, code: &mut CodeFragments, macro_prefix: &str) -> TokenStream {
        if self.0.len() == 1 {
            let label = self.0.keys().next().unwrap();
            let string = String::from_utf8_lossy(&label);
            let kind = code.add_terminal_kind(string.to_string());
            let macro_name = format_ident!("{}terminal", macro_prefix);
            quote!(#macro_name!(#kind, #string))
        } else {
            fn generate_from_trie(
                node: Option<&Node<String>>,
                code: &mut CodeFragments,
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
            quote!(#macro_name!(#(#lexers),*))
        }
    }
}
