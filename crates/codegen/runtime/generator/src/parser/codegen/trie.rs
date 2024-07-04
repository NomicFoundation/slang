use std::collections::BTreeMap;
use std::fmt::Debug;

use codegen_language_definition::model::KeywordDefinition;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::versioned::VersionedQuote as _;
use crate::parser::codegen::KeywordItemAtom;
use crate::parser::grammar::ScannerDefinitionRef;

#[derive(Clone, Debug, Default)]
pub struct Trie<T: Payload> {
    pub subtries: BTreeMap<char, Self>,
    pub key: Option<String>,
    pub payload: Option<T>,
}

impl<T: Payload> Trie<T> {
    pub fn new() -> Self {
        Self {
            subtries: BTreeMap::new(),
            key: None,
            payload: None,
        }
    }

    pub fn insert(&mut self, key: &str, payload: T) {
        let mut node = self;
        for char in key.chars() {
            node = node.subtries.entry(char).or_insert_with(Self::new);
        }
        node.payload = Some(payload);
        node.key = Some(key.to_string());
    }

    // Finds the next node that has either a payload or more than one subtrie
    // It returns the path to that node and the node itself
    pub fn next_interesting_node(&self, prefix: Option<char>) -> (Vec<char>, &Self) {
        let mut path = prefix.map(|c| vec![c]).unwrap_or_default();
        let mut node = self;
        while node.payload.is_none() && node.subtries.len() == 1 {
            let (key, subtrie) = node.subtries.iter().next().unwrap();
            path.push(*key);
            node = subtrie;
        }
        (path, node)
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

        let leaf = trie
            .payload
            .as_ref()
            .map_or_else(T::default_case, T::to_leaf_code);

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

        let default_case = T::default_case();
        if path.is_empty() {
            trie_code
        } else {
            quote! {
                if scan_chars!(input, #(#path),*) {
                    #trie_code
                } else {
                    #default_case
                }
            }
        }
    }
}

/// Used together with [`Trie`]. Represents the payload of a trie node and can be used to customize
/// the emitted code.
///
/// Implemented for [`ScannerDefinitionRef`] and [`KeywordItemAtom`], allows to create
/// tries for both literal scanner definitions and keyword scanners.
pub trait Payload {
    fn to_leaf_code(&self) -> TokenStream;
    fn default_case() -> TokenStream;
}

impl Payload for ScannerDefinitionRef {
    fn to_leaf_code(&self) -> TokenStream {
        let kind = format_ident!("{}", self.name());

        self.version_specifier().to_conditional_code(
            quote! { Some(TerminalKind::#kind) },
            Some(Self::default_case()),
        )
    }

    fn default_case() -> TokenStream {
        quote! { None }
    }
}

impl Payload for KeywordItemAtom {
    fn to_leaf_code(&self) -> TokenStream {
        let kind = format_ident!("{}", self.name);

        let KeywordDefinition {
            enabled, reserved, ..
        } = self.definition();

        let enabled_cond = enabled.as_ref().as_bool_expr();
        let reserved_cond = reserved.as_ref().as_bool_expr();

        // Simplify the emitted code if we trivially know that reserved or enabled is true
        match (&*reserved_cond.to_string(), &*enabled_cond.to_string()) {
            ("true", _) => quote!(KeywordScan::Reserved(TerminalKind::#kind)),
            ("false", "true") => quote!(KeywordScan::Present(TerminalKind::#kind)),
            ("false", _) => quote! {
                if #enabled_cond {
                    KeywordScan::Present(TerminalKind::#kind)
                } else {
                    KeywordScan::Absent
                }
            },
            (_, "false") => quote! {
                if #reserved_cond {
                    KeywordScan::Reserved(TerminalKind::#kind)
                } else {
                    KeywordScan::Absent
                }
            },
            (_, "true") => quote! {
                if #reserved_cond {
                    KeywordScan::Reserved(TerminalKind::#kind)
                } else {
                    KeywordScan::Present(TerminalKind::#kind)
                }
            },
            (reserved, enabled) if reserved == enabled => quote! {
                if #reserved_cond {
                    KeywordScan::Reserved(TerminalKind::#kind)
                } else {
                    KeywordScan::Absent
                }
            },
            _ => quote! {
                if #reserved_cond {
                    KeywordScan::Reserved(TerminalKind::#kind)
                } else if #enabled_cond {
                    KeywordScan::Present(TerminalKind::#kind)
                } else {
                    KeywordScan::Absent
                }
            },
        }
    }

    fn default_case() -> TokenStream {
        quote! { KeywordScan::Absent }
    }
}
