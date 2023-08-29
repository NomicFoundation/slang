use std::collections::BTreeSet;

use codegen_grammar::{ScannerDefinitionNode, ScannerDefinitionRef};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::parser_definition::VersionQualityRangeVecExtensions;

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
            ScannerDefinitionNode::Versioned(body, _, _) => body.literals(accum),
            ScannerDefinitionNode::Literal(string, _) => {
                accum.insert(string.clone());
                true
            }
            ScannerDefinitionNode::Choice(nodes, _) => nodes
                .iter()
                .fold(true, |result, node| node.literals(accum) && result),
            _ => false,
        }
    }

    fn to_scanner_code(&self) -> TokenStream {
        match self {
            ScannerDefinitionNode::Versioned(body, version_quality_ranges, _) => {
                let body = body.to_scanner_code();
                version_quality_ranges.wrap_code(body, Some(quote! { false }))
            }

            ScannerDefinitionNode::Optional(node, _) => {
                let scanner = node.to_scanner_code();
                quote! { scan_optional!(stream, #scanner) }
            }

            ScannerDefinitionNode::ZeroOrMore(node, _) => {
                let scanner = node.to_scanner_code();
                quote! { scan_zero_or_more!(stream, #scanner) }
            }

            ScannerDefinitionNode::OneOrMore(node, _) => {
                let scanner = node.to_scanner_code();
                quote! { scan_one_or_more!(stream, #scanner) }
            }

            ScannerDefinitionNode::NoneOf(string, _) => {
                let chars = string.chars();
                quote! { scan_none_of!(stream, #(#chars),*) }
            }

            ScannerDefinitionNode::NotFollowedBy(node, lookahead, _) => {
                let scanner = node.to_scanner_code();
                let negative_lookahead_scanner = lookahead.to_scanner_code();
                quote! { scan_not_followed_by!(stream, #scanner, #negative_lookahead_scanner) }
            }

            ScannerDefinitionNode::Sequence(nodes, _) => {
                let scanners = nodes
                    .iter()
                    .map(|e| e.to_scanner_code())
                    .collect::<Vec<_>>();
                quote! { scan_sequence!(#(#scanners),*) }
            }

            ScannerDefinitionNode::Choice(nodes, _) => {
                let mut scanners = vec![];
                let mut non_literal_scanners = vec![];
                for node in nodes {
                    if let ScannerDefinitionNode::Literal(string, _) = node {
                        scanners.push(string);
                    } else {
                        non_literal_scanners.push(node.to_scanner_code())
                    }
                }
                scanners.sort();
                let mut scanners = scanners
                    .iter()
                    // We want the longest literals first, so we prefer the longest match
                    .rev()
                    .map(|string| {
                        let chars = string.chars();
                        quote! { scan_chars!(stream, #(#chars),*) }
                    })
                    .collect::<Vec<_>>();
                scanners.extend(non_literal_scanners.into_iter());
                quote! { scan_choice!(stream, #(#scanners),*) }
            }

            ScannerDefinitionNode::CharRange(from, to, _) => {
                quote! { scan_char_range!(stream, #from, #to) }
            }

            ScannerDefinitionNode::Literal(string, _) => {
                let chars = string.chars();
                quote! { scan_chars!(stream, #(#chars),*) }
            }

            ScannerDefinitionNode::ScannerDefinition(scanner_definition, _) => {
                let name = scanner_definition.name();
                let snake_case = name.to_snake_case();
                let scanner_function_name = format_ident!("{snake_case}");
                quote! { self.#scanner_function_name(stream) }
            }
        }
    }
}
