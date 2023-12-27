use codegen_grammar::{
    KeywordScannerDefinitionNode, KeywordScannerDefinitionRef, ScannerDefinitionNode,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::parser_definition::VersionQualityRangeVecExtensions;
use crate::scanner_definition::ScannerDefinitionNodeExtensions;

pub trait KeywordScannerDefinitionExtensions {
    fn to_scanner_code(&self) -> TokenStream;
}

impl KeywordScannerDefinitionExtensions for KeywordScannerDefinitionRef {
    fn to_scanner_code(&self) -> TokenStream {
        let kw_scanners: Vec<_> = self
            .definitions()
            .iter()
            .map(|versioned_kw| {
                let scanner = versioned_kw.value.to_scanner_code();
                let enabled_cond = versioned_kw.enabled.as_bool_expr();
                let reserved_cond = versioned_kw.reserved.as_bool_expr();

                // Simplify the emitted code if we trivially know that reserved or enabled is true
                match (&*reserved_cond.to_string(), &*enabled_cond.to_string()) {
                    ("true", _) => quote! {
                        if #scanner {
                            KeywordScan::Reserved
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    ("false", _) => quote! {
                        if #enabled_cond && #scanner {
                            KeywordScan::Present
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "true") => quote! {
                        if #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved
                            } else {
                                KeywordScan::Present
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "false") => quote! {
                        if #reserved_cond && #scanner {
                            KeywordScan::Reserved
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    _ => quote! {
                        if (#reserved_cond || #enabled_cond) && #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved
                            } else {
                                KeywordScan::Present
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                }

                // quote! {
                //     // Optimize to only attempt scanning if it's enabled or reserved; the (bool) checks are trivial
                //     if (#enabled_cond || #reserved_cond) && #scanner {
                //         if #reserved_cond {
                //             KeywordScan::Reserved
                //         } else {
                //             KeywordScan::Present
                //         }
                //     } else {
                //         KeywordScan::Absent
                //     }
                // }
            })
            .collect();

        match &kw_scanners[..] {
            [] => quote! { KeywordScan::Absent },
            multiple => quote! { scan_keyword_choice!(input, ident, #(#multiple),*) },
        }
    }
}

impl KeywordScannerDefinitionExtensions for KeywordScannerDefinitionNode {
    fn to_scanner_code(&self) -> TokenStream {
        // This is a subset; let's reuse that
        ScannerDefinitionNode::from(self.clone()).to_scanner_code()
    }
}
