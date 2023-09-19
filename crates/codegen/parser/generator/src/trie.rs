use std::{collections::BTreeMap, fmt::Debug};

use codegen_grammar::{ScannerDefinitionNode, ScannerDefinitionRef, VersionQualityRange};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::parser_definition::VersionQualityRangeVecExtensions;

/// Returns concatenated version ranges for applicable scanners (excluding contextual keywords).
/// If the scanner consists only of contextual keywords, then `None` is returned.
fn version_ranges(scanner_def: &ScannerDefinitionNode) -> Option<Vec<VersionQualityRange>> {
    match scanner_def {
        ScannerDefinitionNode::Versioned(node, version_quality_ranges, _) => match node.as_ref() {
            // Ignore versioned contextual keywords, as these are synthesized by the parser,
            // rather than scanned independently.
            ScannerDefinitionNode::ContextualKeyword(..) => None,
            _ => Some(version_quality_ranges.clone()),
        },
        ScannerDefinitionNode::ContextualKeyword(..) => None,
        ScannerDefinitionNode::Choice(nodes, _) => {
            nodes
                .iter()
                .filter_map(version_ranges)
                .fold(None, |acc, item| match acc {
                    None => Some(item),
                    Some(mut acc) => {
                        acc.extend(item);
                        Some(acc)
                    }
                })
        }
        _ => Some(vec![]),
    }
}

#[derive(Clone, Debug, Default)]
pub struct Trie {
    pub subtries: BTreeMap<char, Self>,
    pub key: Option<String>,
    pub payload: Option<ScannerDefinitionRef>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            subtries: BTreeMap::new(),
            key: None,
            payload: None,
        }
    }

    pub fn insert(&mut self, key: &str, payload: ScannerDefinitionRef) {
        let chars = key.chars().collect::<Vec<_>>();
        let mut node = self;
        for i in 0..chars.len() {
            node = node.subtries.entry(chars[i]).or_insert_with(Self::new);
        }
        node.payload = Some(payload);
        node.key = Some(key.to_string());
    }

    // Finds the next node that has either a payload or more than one subtrie
    // It returns the path to that node and the node itself
    pub fn next_interesting_node(&self, prefix: Option<char>) -> (Vec<char>, &Trie) {
        let mut path = prefix.map(|c| vec![c]).unwrap_or_default();
        let mut node = self;
        while node.payload.is_none() && node.subtries.len() == 1 {
            let (key, subtrie) = node.subtries.iter().next().unwrap();
            path.push(*key);
            node = subtrie;
        }
        (path, node)
    }

    pub fn is_empty(&self) -> bool {
        self.subtries.is_empty() && self.payload.is_none()
    }

    pub fn to_scanner_code(&self) -> TokenStream {
        let (path, trie) = self.next_interesting_node(None);

        let branches = trie
            .subtries
            .iter()
            .map(|(c, subtrie)| {
                let child_code = subtrie.to_scanner_code();
                quote! { Some(#c) => #child_code }
            })
            .collect::<Vec<_>>();

        let leaf = if let Some(scanner_definition_ref) = &trie.payload {
            let kind = format_ident!("{}", scanner_definition_ref.name());

            let versioned = version_ranges(scanner_definition_ref.node());

            if branches.is_empty() && !path.is_empty() {
                // This is an optimisation for a common case
                let leaf = quote! { scan_chars!(input, #(#path),*).then_some(TokenKind::#kind) };

                return versioned
                    .map(|range| range.wrap_code(leaf, Some(quote! { None })))
                    .unwrap_or_else(|| quote! { None });
            }

            versioned
                .map(|range| range.wrap_code(quote! {Some (TokenKind::#kind)}, Some(quote! {None})))
                .unwrap_or_else(|| quote! { None })
        } else {
            quote! { None }
        };

        let trie_code = if branches.is_empty() {
            leaf
        } else {
            quote! {
                match input.next() {
                    #(#branches,)*
                    Some(_) => { input.undo(); #leaf }
                    None => #leaf,
                }
            }
        };

        if path.is_empty() {
            trie_code
        } else {
            quote! {
                if scan_chars!(input, #(#path),*) {
                    #trie_code
                } else {
                    None
                }
            }
        }
    }
}
