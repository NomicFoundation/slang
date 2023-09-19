use std::collections::BTreeSet;

use codegen_grammar::{ScannerDefinitionNode, ScannerDefinitionRef, VersionQualityRange};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::parser_definition::VersionQualityRangeVecExtensions;

pub trait ScannerDefinitionExtensions {
    fn to_scanner_code(&self) -> TokenStream;
    fn literals(&self) -> Vec<String>;
    fn as_contextual_keyword(&self) -> Option<ContextualKeywordScanner>;
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
    fn as_contextual_keyword(&self) -> Option<ContextualKeywordScanner> {
        self.node().as_contextual_keyword()
    }
}

pub trait ScannerDefinitionNodeExtensions {
    fn to_scanner_code(&self) -> TokenStream;
    fn literals(&self, accum: &mut BTreeSet<String>) -> bool;
    fn as_contextual_keyword(&self) -> Option<ContextualKeywordScanner>;
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
            ScannerDefinitionNode::ContextualKeyword(string, ..) => {
                accum.insert(string.to_string());
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
                quote! { scan_optional!(input, #scanner) }
            }

            ScannerDefinitionNode::ZeroOrMore(node, _) => {
                let scanner = node.to_scanner_code();
                quote! { scan_zero_or_more!(input, #scanner) }
            }

            ScannerDefinitionNode::OneOrMore(node, _) => {
                let scanner = node.to_scanner_code();
                quote! { scan_one_or_more!(input, #scanner) }
            }

            ScannerDefinitionNode::NoneOf(string, _) => {
                let chars = string.chars();
                quote! { scan_none_of!(input, #(#chars),*) }
            }

            ScannerDefinitionNode::NotFollowedBy(node, lookahead, _) => {
                let scanner = node.to_scanner_code();
                let negative_lookahead_scanner = lookahead.to_scanner_code();
                quote! { scan_not_followed_by!(input, #scanner, #negative_lookahead_scanner) }
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
                        quote! { scan_chars!(input, #(#chars),*) }
                    })
                    .collect::<Vec<_>>();
                scanners.extend(non_literal_scanners.into_iter());
                quote! { scan_choice!(input, #(#scanners),*) }
            }

            ScannerDefinitionNode::CharRange(from, to, _) => {
                quote! { scan_char_range!(input, #from, #to) }
            }

            ScannerDefinitionNode::Literal(string, _) => {
                let chars = string.chars();
                quote! { scan_chars!(input, #(#chars),*) }
            }

            ScannerDefinitionNode::ContextualKeyword(_value, _scanner, _) => {
                // Contextual keywords are not independent scanners but rather
                // upgraded to by the parser.
                quote! { false }
            }

            ScannerDefinitionNode::ScannerDefinition(scanner_definition, _) => {
                let name = scanner_definition.name();
                let snake_case = name.to_snake_case();
                let scanner_function_name = format_ident!("{snake_case}");
                quote! { self.#scanner_function_name(input) }
            }
        }
    }

    /// Returns `Some` if the scanner definition defines a contextual keyword at some version,
    /// `None` otherwise.
    ///
    /// The contextual keywords scanners are not ordinary, as they are only synthesized
    /// from the underlying scanner when parsing, rather than scanned independently.
    fn as_contextual_keyword(&self) -> Option<ContextualKeywordScanner> {
        match self {
            ScannerDefinitionNode::Versioned(node, version_quality_ranges, _) => {
                match node.as_ref() {
                    ScannerDefinitionNode::ContextualKeyword(literal, ident_scanner, _) => {
                        let ident_scanner = match ident_scanner.as_ref() {
                            ScannerDefinitionNode::ScannerDefinition(def, ..) => def.name(),
                            _ => unreachable!("Contextual keywords must be defined using an underlying scanner definition"),
                        };

                        Some(ContextualKeywordScanner {
                            ident_scanner,
                            literal,
                            version_quality_ranges: version_quality_ranges.clone(),
                        })
                    }
                    _ => None,
                }
            }
            ScannerDefinitionNode::ContextualKeyword(literal, ident_scanner, _) => {
                let ident_scanner = match ident_scanner.as_ref() {
                    ScannerDefinitionNode::ScannerDefinition(def, ..) => def.name(),
                    _ => unreachable!("Contextual keywords must be defined using an underlying scanner definition"),
                };

                Some(ContextualKeywordScanner {
                    ident_scanner,
                    literal,
                    version_quality_ranges: vec![],
                })
            }
            ScannerDefinitionNode::Choice(nodes, _) => nodes
                .iter()
                .filter_map(Self::as_contextual_keyword)
                .fold(None, |acc, item| {
                    let Some(mut acc) = acc else { return Some(item) };

                    assert_eq!(
                        acc.ident_scanner, item.ident_scanner,
                        "Contextual keywords must have the same underlying scanner definition"
                    );
                    assert_eq!(
                        acc.literal, item.literal,
                        "Contextual keyword definition must have the same literal"
                    );

                    acc.version_quality_ranges
                        .extend(item.version_quality_ranges);

                    Some(acc)
                }),
            _ => None,
        }
    }
}

pub struct ContextualKeywordScanner {
    /// Name of the underlying scanner
    pub ident_scanner: &'static str,
    /// Literal of the contextual keyword
    pub literal: &'static str,
    /// Version ranges at which the literal is a contextual keyword
    pub version_quality_ranges: Vec<VersionQualityRange>,
}
