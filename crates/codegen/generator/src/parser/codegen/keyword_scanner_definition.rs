use std::rc::Rc;

use language_definition::model;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::scanner_definition::ScannerCodegen as _;
use crate::parser::codegen::versioned::VersionedQuote;

pub trait KeywordScannerDefinitionCodegen {
    fn to_scanner_code(&self) -> TokenStream;
}

impl KeywordScannerDefinitionCodegen for model::KeywordItem {
    fn to_scanner_code(&self) -> TokenStream {
        let name_ident = format_ident!("{}", self.name);
        let terminal_kind = quote! { TerminalKind::#name_ident };

        let kw_scanners: Vec<_> = self
            .definitions
            .iter()
            .map(|versioned_kw| {
                let scanner = versioned_kw.value.to_scanner_code();
                let enabled_cond = versioned_kw.enabled.as_ref().as_bool_expr();
                let reserved_cond = versioned_kw.reserved.as_ref().as_bool_expr();

                // Simplify the emitted code if we trivially know that reserved or enabled is true
                match (&*reserved_cond.to_string(), &*enabled_cond.to_string()) {
                    ("true", _) => quote! {
                        if #scanner {
                            KeywordScan::Reserved(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    ("false", _) => quote! {
                        if #enabled_cond && #scanner {
                            KeywordScan::Present(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "true") => quote! {
                        if #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved(#terminal_kind)
                            } else {
                                KeywordScan::Present(#terminal_kind)
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "false") => quote! {
                        if #reserved_cond && #scanner {
                            KeywordScan::Reserved(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    _ => quote! {
                        if (#reserved_cond || #enabled_cond) && #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved(#terminal_kind)
                            } else {
                                KeywordScan::Present(#terminal_kind)
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                }
            })
            .collect();

        match &kw_scanners[..] {
            [] => quote! { KeywordScan::Absent },
            multiple => quote! { scan_keyword_choice!(input, ident_len, #(#multiple),*) },
        }
    }
}

impl KeywordScannerDefinitionCodegen for model::KeywordValue {
    fn to_scanner_code(&self) -> TokenStream {
        // This is a subset; let's reuse that
        model::Scanner::from(self.clone()).to_scanner_code()
    }
}

/// A newtype wrapper around [`model::KeywordItem`] that only has a single atom value.
///
/// The main usage for this type is to construct a keyword trie, as trie will
/// only work with single atom values and keyword promotion needs to additionally account for
/// keyword reservation, rather than just literal presence.
#[derive(Clone)]
pub struct KeywordItemAtom(Rc<model::KeywordItem>);

impl KeywordItemAtom {
    /// Wraps the keyword scanner definition if it is a single atom value.
    pub fn try_from_def(def: &Rc<model::KeywordItem>) -> Option<Self> {
        match def.definitions[..] {
            [model::KeywordDefinition {
                value: model::KeywordValue::Atom { .. },
                ..
            }] => Some(Self(Rc::clone(def))),
            _ => None,
        }
    }
}

impl std::ops::Deref for KeywordItemAtom {
    type Target = Rc<model::KeywordItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeywordItemAtom {
    pub fn definition(&self) -> &model::KeywordDefinition {
        self.0
            .definitions
            .first()
            .expect("KeywordItemAtom should have exactly one definition")
    }

    /// The single atom value that this keyword item matches.
    pub fn value(&self) -> &str {
        match self.definition() {
            model::KeywordDefinition {
                value: model::KeywordValue::Atom { atom },
                ..
            } => atom,
            _ => unreachable!("KeywordItemAtom should have a single atom value"),
        }
    }
}
