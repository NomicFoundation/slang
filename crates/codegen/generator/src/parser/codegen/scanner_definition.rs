use std::collections::BTreeSet;

use codegen_language_definition::model::{self, Identifier};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::versioned::VersionedQuote;
use crate::parser::grammar::ScannerDefinition;

impl ScannerDefinition for model::TriviaItem {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn to_scanner_code(&self) -> proc_macro2::TokenStream {
        self.scanner.to_scanner_code()
    }

    fn literals(&self) -> Option<BTreeSet<String>> {
        self.scanner.literals()
    }
}

impl ScannerDefinition for model::FragmentItem {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn to_scanner_code(&self) -> proc_macro2::TokenStream {
        VersionedScanner::new(&self.scanner, self.enabled.as_ref()).to_scanner_code()
    }

    fn literals(&self) -> Option<BTreeSet<String>> {
        self.scanner.literals()
    }

    fn version_specifier(&self) -> Option<&model::VersionSpecifier> {
        self.enabled.as_ref()
    }
}

impl ScannerDefinition for model::TokenItem {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn to_scanner_code(&self) -> proc_macro2::TokenStream {
        let defs: Vec<_> = self
            .definitions
            .iter()
            .map(|def| VersionedScanner::new(&def.scanner, def.enabled.as_ref()))
            .collect();

        match defs.len() {
            0 => panic!("Token {} has no definitions", self.name),
            1 => defs.into_iter().next().unwrap().to_scanner_code(),
            _ => choice_to_scanner_code(&defs),
        }
    }

    fn literals(&self) -> Option<BTreeSet<String>> {
        self.definitions
            .iter()
            .try_fold(BTreeSet::new(), |mut acc, def| {
                let literals = def.scanner.literals()?;
                acc.extend(literals);
                Some(acc)
            })
    }
}

pub(crate) trait ScannerCodegen {
    /// Quotes the matching Rust scanner code.
    fn to_scanner_code(&self) -> TokenStream;
    /// Whether the scanner is an atom, and if so, returns the atom.
    fn as_atom(&self) -> Option<&str>;
    /// Returns a set of literals that this scanner can match.
    fn literals(&self) -> Option<BTreeSet<String>>;
}

/// Enhances the [`model::Scanner`] with version information.
///
/// Used to generate code for scanners that are versioned, i.e. wrapped in conditional blocks.
struct VersionedScanner<'a> {
    scanner: &'a model::Scanner,
    enabled: Option<&'a model::VersionSpecifier>,
}

impl ScannerCodegen for VersionedScanner<'_> {
    fn to_scanner_code(&self) -> TokenStream {
        let scanner = self.scanner.to_scanner_code();
        self.enabled
            .to_conditional_code(scanner, Some(quote! { false }))
    }

    fn as_atom(&self) -> Option<&str> {
        None
    }

    fn literals(&self) -> Option<BTreeSet<String>> {
        self.scanner.literals()
    }
}

impl<'a> VersionedScanner<'a> {
    fn new(scanner: &'a model::Scanner, enabled: Option<&'a model::VersionSpecifier>) -> Self {
        Self { scanner, enabled }
    }
}

impl ScannerCodegen for model::Scanner {
    fn to_scanner_code(&self) -> TokenStream {
        match self {
            model::Scanner::Optional { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_optional!(input, #scanner) }
            }
            model::Scanner::ZeroOrMore { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_zero_or_more!(input, #scanner) }
            }

            model::Scanner::OneOrMore { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_one_or_more!(input, #scanner) }
            }
            model::Scanner::Not { chars } => {
                let chars = chars.iter();
                quote! { scan_none_of!(input, #(#chars),*) }
            }
            model::Scanner::TrailingContext {
                scanner: node,
                not_followed_by: lookahead,
            } => {
                let scanner = node.to_scanner_code();
                let negative_lookahead_scanner = lookahead.to_scanner_code();
                quote! { scan_not_followed_by!(input, #scanner, #negative_lookahead_scanner) }
            }
            model::Scanner::Sequence { scanners } => {
                let scanners = scanners
                    .iter()
                    .map(|e| e.to_scanner_code())
                    .collect::<Vec<_>>();
                quote! { scan_sequence!(#(#scanners),*) }
            }
            model::Scanner::Choice { scanners: nodes } => choice_to_scanner_code(nodes),

            model::Scanner::Range {
                inclusive_start: start,
                inclusive_end: end,
            } => {
                quote! { scan_char_range!(input, #start..=#end) }
            }
            model::Scanner::Atom { atom } => {
                let chars = atom.chars();
                quote! { scan_chars!(input, #(#chars),*) }
            }

            model::Scanner::Fragment { reference } => {
                let snake_case = reference.to_snake_case();
                let scanner_function_name = format_ident!("{snake_case}");
                quote! { self.#scanner_function_name(input) }
            }
        }
    }

    fn as_atom(&self) -> Option<&str> {
        match self {
            model::Scanner::Atom { atom } => Some(atom),
            _ => None,
        }
    }

    fn literals(&self) -> Option<BTreeSet<String>> {
        fn accumulate(scanner: &model::Scanner, accum: &mut BTreeSet<String>) -> bool {
            match scanner {
                model::Scanner::Atom { atom } => {
                    accum.insert(atom.clone());
                    true
                }
                model::Scanner::Choice { scanners } => scanners
                    .iter()
                    .fold(true, |result, node| accumulate(node, accum) && result),
                _ => false,
            }
        }

        let mut literals = BTreeSet::default();
        accumulate(self, &mut literals).then_some(literals)
    }
}

fn choice_to_scanner_code<T: ScannerCodegen>(nodes: &[T]) -> TokenStream {
    let mut scanners = vec![];
    let mut non_literal_scanners = vec![];
    for node in nodes {
        if let Some(atom) = node.as_atom() {
            scanners.push(atom);
        } else {
            non_literal_scanners.push(node.to_scanner_code());
        }
    }
    scanners.sort_unstable();
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
