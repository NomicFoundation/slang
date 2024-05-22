//! Defines parser code generation for the language grammar.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use codegen_language_definition::model::Language;
use quote::{format_ident, quote};
use semver::Version;
use serde::Serialize;

mod grammar;
mod keyword_scanner_definition;
mod parser_definition;
mod precedence_parser_definition;
mod scanner_definition;
mod trie;

use grammar::{
    Grammar, GrammarVisitor, KeywordScannerAtomic, KeywordScannerDefinitionRef,
    ParserDefinitionNode, ParserDefinitionRef, PrecedenceParserDefinitionRef,
    ScannerDefinitionNode, ScannerDefinitionRef, TriviaParserDefinitionRef,
};
use keyword_scanner_definition::KeywordScannerDefinitionExtensions as _;
use parser_definition::ParserDefinitionExtensions as _;
use precedence_parser_definition::PrecedenceParserDefinitionExtensions as _;
use scanner_definition::ScannerDefinitionExtensions as _;
use trie::Trie;

#[derive(Default, Serialize)]
pub struct ParserModel {
    /// Defines the `Language::SUPPORTED_VERSIONS` field.
    all_versions: BTreeSet<Version>,
    /// Constructs inner `Language` the state to evaluate the version-dependent branches.
    referenced_versions: BTreeSet<Version>,

    /// Defines the `NonTerminalKind` enum variants.
    nonterminal_kinds: BTreeSet<&'static str>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<&'static str>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<&'static str>,
    /// Defines `NodeLabel` enum variants.
    labels: BTreeSet<String>,

    /// Defines the top-level scanner functions in `Language`.
    scanner_functions: BTreeMap<&'static str, String>, // (name of scanner, code)
    // Defines the `LexicalContext(Type)` enum and type-level variants.
    scanner_contexts: BTreeMap<&'static str, ScannerContext>,
    /// Defines the top-level compound scanners used when lexing in `Language`.
    keyword_compound_scanners: BTreeMap<&'static str, String>, // (name of the KW scanner, code)

    /// Defines the top-level parser functions in `Language`.
    parser_functions: BTreeMap<&'static str, String>, // (name of parser, code)
    /// Defines the top-level trivia parser functions in `Language`.
    trivia_parser_functions: BTreeMap<&'static str, String>, // (name of parser, code)

    // Internal state:
    /// Makes sure to codegen the scanner functions that are referenced by other scanners.
    #[serde(skip)]
    top_level_scanner_names: BTreeSet<&'static str>,
    /// Lookup table for all scanners; used to generate trie scanners.
    #[serde(skip)]
    all_scanners: BTreeMap<&'static str, ScannerDefinitionRef>,
    /// The current context of a parent scanner/parser being processed.
    #[serde(skip)]
    current_context_name: &'static str,
}

#[derive(Default, Serialize)]
struct ScannerContext {
    /// Rust code for the trie scanner that matches literals.
    literal_scanner: String,
    /// Names of the compound scanners that are keywords.
    // Values (Rust code) is only used to generate the top-level `keyword_compound_scanners`.
    keyword_compound_scanners: BTreeMap<&'static str, String>,
    /// Rust code for the trie scanner that matches keywords
    keyword_trie_scanner: String,
    /// Names of the scanners for identifiers that can be promoted to keywords.
    promotable_identifier_scanners: BTreeSet<&'static str>,
    /// Names of the scanners that are compound (do not consist of only literals).
    compound_scanner_names: Vec<&'static str>,
    /// Set of delimiter pairs for this context that are used in delimited error recovery.
    delimiters: BTreeMap<&'static str, &'static str>,
    // Internal state:
    #[serde(skip)]
    scanner_definitions: BTreeSet<&'static str>,
    #[serde(skip)]
    keyword_scanner_defs: BTreeMap<&'static str, KeywordScannerDefinitionRef>,
}

impl ParserModel {
    pub fn from_language(language: &Rc<Language>) -> Self {
        let grammar = Grammar::from_dsl_v2(language);

        let mut model = Self::default();
        grammar.accept_visitor(&mut model);

        model
    }

    fn set_current_context(&mut self, name: &'static str) {
        self.current_context_name = name;
        self.scanner_contexts.entry(name).or_default();
    }

    fn current_context(&mut self) -> &mut ScannerContext {
        self.scanner_contexts
            .get_mut(&self.current_context_name)
            .expect("context must be set with `set_current_context`")
    }
}

impl GrammarVisitor for ParserModel {
    fn grammar_leave(&mut self, _grammar: &Grammar) {
        // Expose the scanner functions that...
        self.scanner_functions = self
            .all_scanners
            .iter()
            .filter(|(name, scanner)| {
                // are compound (do not consist of only literals)
                scanner.literals().is_empty() ||
                // but make sure to also include a scanner that is referenced by other scanners, even if not compound
                !self.top_level_scanner_names.contains(*name)
            })
            .map(|(name, scanner)| (*name, scanner.to_scanner_code().to_string()))
            .collect();

        for context in self.scanner_contexts.values_mut() {
            let mut literal_trie = Trie::new();

            for scanner_name in &context.scanner_definitions {
                let scanner = &self.all_scanners[*scanner_name];

                let literals = scanner.literals();
                if literals.is_empty() {
                    context.compound_scanner_names.push(scanner_name);
                } else {
                    for literal in literals {
                        literal_trie.insert(&literal, Rc::clone(scanner));
                    }
                }
            }

            context.literal_scanner = literal_trie.to_scanner_code().to_string();

            context.promotable_identifier_scanners = context
                .keyword_scanner_defs
                .values()
                .map(|def| def.identifier_scanner())
                .collect();

            let mut keyword_trie = Trie::new();
            for (name, def) in &context.keyword_scanner_defs {
                match KeywordScannerAtomic::try_from_def(def) {
                    Some(atomic) => keyword_trie.insert(atomic.value(), atomic.clone()),
                    None => {
                        context
                            .keyword_compound_scanners
                            .insert(name, def.to_scanner_code().to_string());
                    }
                }
            }

            context.keyword_trie_scanner = keyword_trie.to_scanner_code().to_string();
        }

        // Collect all of the keyword scanners into a single list to be defined at top-level
        self.keyword_compound_scanners = self
            .scanner_contexts
            .values()
            .flat_map(|context| {
                context
                    .keyword_compound_scanners
                    .iter()
                    .map(|(name, code)| (*name, code.clone()))
            })
            .collect();

        // Make sure empty strings are not there
        self.labels.remove("");
        // These are built-in and already pre-defined
        // _SLANG_INTERNAL_RESERVED_NODE_LABELS_ (keep in sync)
        self.labels.remove("item");
        self.labels.remove("variant");
        self.labels.remove("separator");
        self.labels.remove("operand");
        self.labels.remove("left_operand");
        self.labels.remove("right_operand");
        self.labels.remove("leading_trivia");
        self.labels.remove("trailing_trivia");

        // Just being anal about tidying up :)
        self.all_scanners.clear();
        self.current_context_name = "";
    }

    fn scanner_definition_enter(&mut self, scanner: &ScannerDefinitionRef) {
        self.all_scanners.insert(scanner.name(), Rc::clone(scanner));
    }

    fn keyword_scanner_definition_enter(&mut self, scanner: &KeywordScannerDefinitionRef) {
        for def in scanner.definitions() {
            let versions = def.enabled.iter().chain(def.reserved.iter());

            self.referenced_versions.extend(
                versions
                    .map(|vqr| &vqr.from)
                    // "Removed from 0.0.0" is an alias for "never"; it's never directly checked
                    .filter(|v| *v != &Version::new(0, 0, 0))
                    .cloned(),
            );
        }
    }

    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context());
        let trivia_scanners = {
            use crate::parser::grammar::visitor::Visitable;

            #[derive(Default)]
            struct CollectTriviaScanners {
                scanner_names: BTreeSet<&'static str>,
            }
            impl crate::parser::grammar::visitor::GrammarVisitor for CollectTriviaScanners {
                fn scanner_definition_enter(&mut self, node: &ScannerDefinitionRef) {
                    self.scanner_names.insert(node.name());
                }
            }

            let mut visitor = CollectTriviaScanners::default();
            parser.node().accept_visitor(&mut visitor);
            visitor.scanner_names
        };
        self.trivia_scanner_names.extend(trivia_scanners);

        self.trivia_parser_functions
            .insert(parser.name(), parser.to_parser_code().to_string());
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        // Have to set this regardless so that we can collect referenced scanners
        self.set_current_context(parser.context());
        if !parser.is_inline() {
            self.nonterminal_kinds.insert(parser.name());
            let code = parser.to_parser_code();
            self.parser_functions.insert(
                parser.name(),
                {
                    let nonterminal_kind = format_ident!("{}", parser.name());
                    quote! { #code.with_kind(NonTerminalKind::#nonterminal_kind) }
                }
                .to_string(),
            );
        }
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context());
        self.nonterminal_kinds.insert(parser.name());
        for (_, name, _) in &parser.node().operators {
            self.nonterminal_kinds.insert(name);
        }

        // While it's not common to parse a precedence expression as a standalone rule,
        // we generate a function for completeness.
        for (name, code) in parser.to_precedence_expression_parser_code() {
            self.parser_functions.insert(name, code.to_string());
        }

        self.parser_functions.insert(
            parser.name(),
            {
                let code = parser.to_parser_code();
                let nonterminal_kind = format_ident!("{}", parser.name());
                quote! { #code.with_kind(NonTerminalKind::#nonterminal_kind) }
            }
            .to_string(),
        );
    }

    fn scanner_definition_node_enter(&mut self, node: &ScannerDefinitionNode) {
        if let ScannerDefinitionNode::Versioned(_, version_quality_ranges) = node {
            for vqr in version_quality_ranges {
                self.referenced_versions.insert(vqr.from.clone());
            }
        }
    }

    fn parser_definition_node_enter(&mut self, node: &ParserDefinitionNode) {
        match node {
            ParserDefinitionNode::Versioned(_, version_quality_ranges) => {
                for vqr in version_quality_ranges {
                    self.referenced_versions.insert(vqr.from.clone());
                }
            }
            ParserDefinitionNode::ScannerDefinition(scanner) => {
                self.top_level_scanner_names.insert(scanner.name());
                self.terminal_kinds.insert(scanner.name());

                self.current_context()
                    .scanner_definitions
                    .insert(scanner.name());
            }
            ParserDefinitionNode::KeywordScannerDefinition(scanner) => {
                self.terminal_kinds.insert(scanner.name());

                self.current_context()
                    .keyword_scanner_defs
                    .insert(scanner.name(), Rc::clone(scanner));
            }

            // Collect labels:
            ParserDefinitionNode::Choice(choice) => {
                self.labels.insert(choice.label.clone());
            }
            ParserDefinitionNode::Sequence(sequence) => {
                for node in sequence {
                    self.labels.insert(node.label.clone());
                }
            }
            ParserDefinitionNode::SeparatedBy(item, separator) => {
                self.labels.insert(item.label.clone());
                self.labels.insert(separator.label.clone());
            }
            ParserDefinitionNode::TerminatedBy(_, terminator) => {
                self.labels.insert(terminator.label.clone());
            }

            // Collect delimiters for each context
            ParserDefinitionNode::DelimitedBy(open, _, close, ..) => {
                self.labels.insert(open.label.clone());
                self.labels.insert(close.label.clone());

                let (open, close) = match (open.as_ref(), close.as_ref()) {
                    (
                        ParserDefinitionNode::ScannerDefinition(open, ..),
                        ParserDefinitionNode::ScannerDefinition(close, ..),
                    ) => (open.name(), close.name()),
                    _ => panic!("DelimitedBy must be delimited by scanners"),
                };

                let delimiters = &mut self.current_context().delimiters;

                assert!(
                    delimiters.get(close).is_none(),
                    "Cannot use a closing delimiter as an opening one"
                );
                delimiters.insert(open, close);
            }
            _ => {}
        };
    }
}
