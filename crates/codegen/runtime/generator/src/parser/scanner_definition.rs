use std::collections::BTreeSet;

use codegen_grammar::{ScannerDefinitionNode, ScannerDefinitionRef};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::parser_definition::VersionQualityRangeVecExtensions;

pub trait ScannerDefinitionExtensions {
    fn to_scanner_code(&self) -> TokenStream;
    fn literals(&self) -> Vec<String>;
}

impl ScannerDefinitionExtensions for ScannerDefinitionRef {
    fn to_scanner_code(&self) -> TokenStream {
        self.node().to_scanner_code()
    }
    fn literals(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        if self.node().literals(&mut result) {
            result.into_iter().collect()
        } else {
            vec![]
        }
    }
}

pub trait ScannerDefinitionNodeExtensions {
    fn to_scanner_code(&self) -> TokenStream;
    fn literals(&self, accum: &mut BTreeSet<String>) -> bool;
}

impl ScannerDefinitionNodeExtensions for ScannerDefinitionNode {
    // Returns true if this is nothing but a set of literals
    fn literals(&self, accum: &mut BTreeSet<String>) -> bool {
        match self {
            ScannerDefinitionNode::Versioned(body, _) => body.literals(accum),
            ScannerDefinitionNode::Literal(string) => {
                accum.insert(string.clone());
                true
            }
            ScannerDefinitionNode::Choice(nodes) => nodes
                .iter()
                .fold(true, |result, node| node.literals(accum) && result),
            _ => false,
        }
    }

    fn to_scanner_code(&self) -> TokenStream {
        match self {
            ScannerDefinitionNode::Versioned(body, version_quality_ranges) => {
                let body = body.to_scanner_code();
                version_quality_ranges.wrap_code(body, Some(quote! { false }))
            }

            ScannerDefinitionNode::Optional(node) => {
                let scanner = node.to_scanner_code();
                quote! { scan_optional!(input, #scanner) }
            }

            ScannerDefinitionNode::ZeroOrMore(node) => {
                let scanner = node.to_scanner_code();
                quote! { scan_zero_or_more!(input, #scanner) }
            }

            ScannerDefinitionNode::OneOrMore(node) => {
                let scanner = node.to_scanner_code();
                quote! { scan_one_or_more!(input, #scanner) }
            }

            ScannerDefinitionNode::NoneOf(string) => {
                let chars = string.chars();
                quote! { scan_none_of!(input, #(#chars),*) }
            }

            ScannerDefinitionNode::NotFollowedBy(node, lookahead) => {
                let scanner = node.to_scanner_code();
                let negative_lookahead_scanner = lookahead.to_scanner_code();
                quote! { scan_not_followed_by!(input, #scanner, #negative_lookahead_scanner) }
            }

            ScannerDefinitionNode::Sequence(nodes) => {
                let scanners = nodes
                    .iter()
                    .map(|e| e.to_scanner_code())
                    .collect::<Vec<_>>();
                quote! { scan_sequence!(#(#scanners),*) }
            }

            ScannerDefinitionNode::Choice(nodes) => {
                let mut scanners = vec![];
                let mut non_literal_scanners = vec![];
                for node in nodes {
                    if let ScannerDefinitionNode::Literal(string) = node {
                        scanners.push(string);
                    } else {
                        non_literal_scanners.push(node.to_scanner_code());
                    }
                }
                scanners.sort();
                let mut scanners = scanners
                    .iter()
                    // We want the longest literals first, so we prefer the longest match
                    .rev()
                    .map(|string| {
                        let chars = string.chars();
                        quote! { scan_chars!(input, #(#chars),*) }
                    })
                    .collect::<Vec<_>>();
                scanners.extend(non_literal_scanners);
                quote! { scan_choice!(input, #(#scanners),*) }
            }

            ScannerDefinitionNode::CharRange(from, to) => {
                quote! { scan_char_range!(input, #from..=#to) }
            }

            ScannerDefinitionNode::Literal(string) => {
                let chars = string.chars();
                quote! { scan_chars!(input, #(#chars),*) }
            }

            ScannerDefinitionNode::ScannerDefinition(scanner_definition) => {
                let name = scanner_definition.name();
                let snake_case = name.to_snake_case();
                let scanner_function_name = format_ident!("{snake_case}");
                quote! { self.#scanner_function_name(input) }
            }
        }
    }
}
