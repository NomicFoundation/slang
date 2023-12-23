use codegen_grammar::{
    KeywordScannerDefinitionNode, KeywordScannerDefinitionRef, ScannerDefinitionNode,
};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser_definition::VersionQualityRangeVecExtensions;
use crate::scanner_definition::ScannerDefinitionNodeExtensions;

pub trait KeywordScannerDefinitionExtensions {
    fn to_scanner_code(&self) -> TokenStream;
}

impl KeywordScannerDefinitionExtensions for KeywordScannerDefinitionRef {
    fn to_scanner_code(&self) -> TokenStream {
        let ident_scanner = self.identifier_scanner();

        let kw_scanners: Vec<_> = self
            .definitions()
            .iter()
            .map(|versioned_kw| {
                let scanner = versioned_kw.value.to_scanner_code();
                let enabled_cond = versioned_kw.enabled.as_bool_expr();
                let reserved_cond = versioned_kw.reserved.as_bool_expr();

                quote! {
                    if !#scanner {
                        KeywordScan::Absent
                    } else if #reserved_cond {
                        KeywordScan::Reserved
                    } else if #enabled_cond {
                        // The keyword is enabled (reachable in this version), so is contextually present
                        KeywordScan::Present
                    } else {
                        // Should be picked up by a catch-all underlying scanner
                        KeywordScan::Absent
                    }
                }
            })
            .collect();

        let underlying_scanner = format_ident!("{}", ident_scanner.to_snake_case());
        quote! {
            // First scan using the underlying scanner (for completeness) and then check if it
            // matches the keyword (and whether it's reserved)
            let save = input.position();
            let scanned = self.#underlying_scanner(input);
            input.set_position(save);
            if !scanned {
                return KeywordScan::Absent;
            }

            scan_keyword_choice!(input, #(#kw_scanners),*)
        }
    }
}

impl KeywordScannerDefinitionExtensions for KeywordScannerDefinitionNode {
    fn to_scanner_code(&self) -> TokenStream {
        // This is a subset; let's reuse that
        ScannerDefinitionNode::from(self.clone()).to_scanner_code()
    }
}
