//! Defines parser code generation for the language grammar.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use codegen_language_definition::model::{Identifier, Language, VersionSpecifier};
use quote::{format_ident, quote};
use semver::Version;
use serde::Serialize;

mod codegen;
mod grammar;

use codegen::{
    KeywordScannerDefinitionCodegen as _, ParserDefinitionCodegen as _,
    PrecedenceParserDefinitionCodegen as _, ScannerDefinitionCodegen as _, Trie,
};
use grammar::{
    Grammar, GrammarVisitor, KeywordScannerAtomic, KeywordScannerDefinitionRef,
    ParserDefinitionNode, ParserDefinitionRef, PrecedenceParserDefinitionRef,
    ScannerDefinitionNode, ScannerDefinitionRef, TriviaParserDefinitionRef,
};

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Default, Clone)]
struct RustCode(String);

#[derive(Default, Serialize)]
pub struct ParserModel {
    /// Constructs inner `Language` the state to evaluate the version-dependent branches.
    referenced_versions: BTreeSet<Version>,

    /// Defines the `NonTerminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Identifier>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<Identifier>,
    /// Defines `NodeLabel` enum variants.
    labels: BTreeSet<String>,

    /// Defines the top-level scanner functions in `Language`.
    scanner_functions: BTreeMap<Identifier, RustCode>, // (name of scanner, code)
    // Defines the `LexicalContext(Type)` enum and type-level variants.
    scanner_contexts: BTreeMap<Identifier, ScannerContextModel>,
    /// Defines the top-level compound scanners used when lexing in `Language`.
    keyword_compound_scanners: BTreeMap<Identifier, RustCode>, // (name of the KW scanner, code)

    /// Defines the top-level parser functions in `Language`.
    parser_functions: BTreeMap<Identifier, RustCode>, // (name of parser, code)
    /// Defines the top-level trivia parser functions in `Language`.
    trivia_parser_functions: BTreeMap<Identifier, RustCode>, // (name of parser, code)
}

#[derive(Default, Serialize)]
struct ScannerContextModel {
    /// Rust code for the trie scanner that matches literals.
    literal_scanner: RustCode,
    /// Names of the compound scanners that are keywords.
    // Values (Rust code) is only used to generate the top-level `keyword_compound_scanners`.
    keyword_compound_scanners: BTreeMap<Identifier, RustCode>,
    /// Rust code for the trie scanner that matches keywords
    keyword_trie_scanner: RustCode,
    /// Names of the scanners for identifiers that can be promoted to keywords.
    promotable_identifier_scanners: BTreeSet<Identifier>,
    /// Names of the scanners that are compound (do not consist of only literals).
    compound_scanner_names: Vec<Identifier>,
    /// Set of delimiter pairs for this context that are used in delimited error recovery.
    delimiters: BTreeMap<Identifier, Identifier>,
}

#[derive(Default)]
struct ParserAccumulatorState {
    /// Constructs inner `Language` the state to evaluate the version-dependent branches.
    referenced_versions: BTreeSet<Version>,

    /// Defines the `NonTerminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Identifier>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<Identifier>,
    /// Defines `NodeLabel` enum variants.
    labels: BTreeSet<String>,

    // Defines the `LexicalContext(Type)` enum and type-level variants.
    scanner_contexts: BTreeMap<Identifier, ScannerContextAccumulatorState>,

    /// Defines the top-level parser functions in `Language`.
    parser_functions: BTreeMap<Identifier, RustCode>, // (name of parser, code)
    /// Defines the top-level trivia parser functions in `Language`.
    trivia_parser_functions: BTreeMap<Identifier, RustCode>, // (name of parser, code)

    /// Makes sure to codegen the scanner functions that are referenced by other scanners.
    top_level_scanner_names: BTreeSet<Identifier>,
    /// Lookup table for all scanners; used to generate trie scanners.
    all_scanners: BTreeMap<Identifier, ScannerDefinitionRef>,
    /// The current context of a parent scanner/parser being processed.
    current_context_name: Option<Identifier>,
}

#[derive(Default)]
struct ScannerContextAccumulatorState {
    /// Set of delimiter pairs for this context that are used in delimited error recovery.
    delimiters: BTreeMap<Identifier, Identifier>,
    scanner_definitions: BTreeSet<Identifier>,
    keyword_scanner_defs: BTreeMap<Identifier, KeywordScannerDefinitionRef>,
}

impl ParserModel {
    pub fn from_language(language: &Rc<Language>) -> Self {
        // First, we construct the DSLv1 model from the DSLv2 definition...
        let grammar = Grammar::from_dsl_v2(language);
        // ...which we then transform into the parser model
        let mut acc = ParserAccumulatorState::default();
        grammar.accept_visitor(&mut acc);

        acc.into_model()
    }
}

impl ParserAccumulatorState {
    fn set_current_context(&mut self, name: Identifier) {
        self.current_context_name = Some(name.clone());
        self.scanner_contexts.entry(name).or_default();
    }

    fn current_context(&mut self) -> &mut ScannerContextAccumulatorState {
        self.scanner_contexts
            .get_mut(self.current_context_name.as_ref().unwrap())
            .expect("context must be set with `set_current_context`")
    }

    fn into_model(mut self) -> ParserModel {
        let contexts = self
            .scanner_contexts
            .into_iter()
            .map(|(name, context)| {
                let mut acc = ScannerContextModel {
                    delimiters: context.delimiters,
                    ..Default::default()
                };

                // Process literals into trie and compound scanners
                let mut literal_trie = Trie::new();

                for scanner_name in &context.scanner_definitions {
                    let scanner = &self.all_scanners[scanner_name];

                    let literals = scanner.literals();
                    if literals.is_empty() {
                        acc.compound_scanner_names.push(scanner_name.clone());
                    } else {
                        for literal in literals {
                            literal_trie.insert(&literal, Rc::clone(scanner));
                        }
                    }
                }
                acc.literal_scanner = RustCode(literal_trie.to_scanner_code().to_string());

                acc.promotable_identifier_scanners = context
                    .keyword_scanner_defs
                    .values()
                    .map(|def| def.identifier_scanner().clone())
                    .collect();

                let mut keyword_trie = Trie::new();
                for (name, def) in &context.keyword_scanner_defs {
                    match KeywordScannerAtomic::try_from_def(def) {
                        Some(atomic) => keyword_trie.insert(atomic.value(), atomic.clone()),
                        None => {
                            acc.keyword_compound_scanners
                                .insert(name.clone(), RustCode(def.to_scanner_code().to_string()));
                        }
                    }
                }

                acc.keyword_trie_scanner = RustCode(keyword_trie.to_scanner_code().to_string());

                (name, acc)
            })
            .collect::<BTreeMap<_, _>>();

        // Expose the scanner functions that...
        let scanner_functions = self
            .all_scanners
            .iter()
            .filter(|(name, scanner)| {
                // are compound (do not consist of only literals)
                scanner.literals().is_empty() ||
                // but make sure to also include a scanner that is referenced by other scanners, even if not compound
                !self.top_level_scanner_names.contains(*name)
            })
            .map(|(name, scanner)| {
                (
                    name.clone(),
                    RustCode(scanner.to_scanner_code().to_string()),
                )
            })
            .collect();

        // Collect all of the keyword scanners into a single list to be defined at top-level
        let keyword_compound_scanners = contexts
            .values()
            .flat_map(|context| {
                context
                    .keyword_compound_scanners
                    .iter()
                    .map(|(name, code)| (name.clone(), code.clone()))
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

        ParserModel {
            referenced_versions: self.referenced_versions,
            nonterminal_kinds: self.nonterminal_kinds,
            terminal_kinds: self.terminal_kinds,
            trivia_scanner_names: self.trivia_scanner_names,
            labels: self.labels,
            parser_functions: self.parser_functions,
            trivia_parser_functions: self.trivia_parser_functions,
            // These are derived from the accumulated state
            scanner_contexts: contexts,
            scanner_functions,
            keyword_compound_scanners,
        }
    }
}

impl GrammarVisitor for ParserAccumulatorState {
    fn scanner_definition_enter(&mut self, scanner: &ScannerDefinitionRef) {
        self.all_scanners
            .insert(scanner.name().clone(), Rc::clone(scanner));
    }

    fn keyword_scanner_definition_enter(&mut self, scanner: &KeywordScannerDefinitionRef) {
        for def in scanner.definitions() {
            let specifiers = def.enabled.iter().chain(def.reserved.iter());

            self.referenced_versions
                .extend(specifiers.flat_map(VersionSpecifier::versions).cloned());
        }
    }

    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context().clone());
        let trivia_scanners = {
            use crate::parser::grammar::visitor::Visitable;

            #[derive(Default)]
            struct CollectTriviaScanners {
                scanner_names: BTreeSet<Identifier>,
            }
            impl crate::parser::grammar::visitor::GrammarVisitor for CollectTriviaScanners {
                fn scanner_definition_enter(&mut self, node: &ScannerDefinitionRef) {
                    self.scanner_names.insert(node.name().clone());
                }
            }

            let mut visitor = CollectTriviaScanners::default();
            parser.node().accept_visitor(&mut visitor);
            visitor.scanner_names
        };
        self.trivia_scanner_names.extend(trivia_scanners);

        self.trivia_parser_functions.insert(
            parser.name().clone(),
            RustCode(parser.to_parser_code().to_string()),
        );
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        // Have to set this regardless so that we can collect referenced scanners
        self.set_current_context(parser.context().clone());
        if !parser.is_inline() {
            self.nonterminal_kinds.insert(parser.name().clone());
            let code = parser.to_parser_code();
            self.parser_functions.insert(
                parser.name().clone(),
                RustCode(
                    {
                        let nonterminal_kind = format_ident!("{}", parser.name());
                        quote! { #code.with_kind(NonTerminalKind::#nonterminal_kind) }
                    }
                    .to_string(),
                ),
            );
        }
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context().clone());
        self.nonterminal_kinds.insert(parser.name().clone());
        for (_, name, _) in &parser.node().operators {
            self.nonterminal_kinds.insert(name.clone());
        }

        // While it's not common to parse a precedence expression as a standalone rule,
        // we generate a function for completeness.
        for (name, code) in parser.to_precedence_expression_parser_code() {
            self.parser_functions
                .insert(name.clone(), RustCode(code.to_string()));
        }

        self.parser_functions.insert(
            parser.name().clone(),
            RustCode(
                {
                    let code = parser.to_parser_code();
                    let nonterminal_kind = format_ident!("{}", parser.name());
                    quote! { #code.with_kind(NonTerminalKind::#nonterminal_kind) }
                }
                .to_string(),
            ),
        );
    }

    fn scanner_definition_node_enter(&mut self, node: &ScannerDefinitionNode) {
        if let ScannerDefinitionNode::Versioned(_, version_specifier) = node {
            self.referenced_versions
                .extend(version_specifier.versions().cloned());
        }
    }

    fn parser_definition_node_enter(&mut self, node: &ParserDefinitionNode) {
        match node {
            ParserDefinitionNode::Versioned(_, version_specifier) => {
                self.referenced_versions
                    .extend(version_specifier.versions().cloned());
            }
            ParserDefinitionNode::ScannerDefinition(scanner) => {
                self.top_level_scanner_names.insert(scanner.name().clone());
                self.terminal_kinds.insert(scanner.name().clone());

                self.current_context()
                    .scanner_definitions
                    .insert(scanner.name().clone());
            }
            ParserDefinitionNode::KeywordScannerDefinition(scanner) => {
                self.terminal_kinds.insert(scanner.name().clone());

                self.current_context()
                    .keyword_scanner_defs
                    .insert(scanner.name().clone(), Rc::clone(scanner));
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
                delimiters.insert(open.clone(), close.clone());
            }
            _ => {}
        };
    }
}
