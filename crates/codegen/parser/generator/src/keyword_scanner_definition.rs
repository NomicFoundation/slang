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

                quote! {
                    // Optimize to only attempt scanning if it's enabled or reserved; the (bool) checks are trivial
                    if (#enabled_cond || #reserved_cond) && #scanner {
                        if #reserved_cond {
                            KeywordScan::Reserved
                        } else {
                            KeywordScan::Present
                        }
                    } else {
                        KeywordScan::Absent
                    }
                }
            })
            .collect();

        match &kw_scanners[..] {
            [] => quote! { KeywordScan::Absent },
            [scanner] => scanner.clone(),
            multiple => quote! { scan_keyword_choice!(input, #(#multiple),*) },
        }
    }
}

impl KeywordScannerDefinitionExtensions for KeywordScannerDefinitionNode {
    fn to_scanner_code(&self) -> TokenStream {
        // This is a subset; let's reuse that
        ScannerDefinitionNode::from(self.clone()).to_scanner_code()
    }
}
