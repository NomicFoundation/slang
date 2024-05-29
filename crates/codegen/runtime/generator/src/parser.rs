//! Defines parser code generation for the language grammar.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use codegen_language_definition::model::{Identifier, Item, Language};
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
    ParserDefinitionNode, ParserDefinitionRef, PrecedenceParserDefinitionRef, ScannerDefinitionRef,
    TriviaParserDefinitionRef,
};

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Default, Clone)]
struct RustCode(String);

#[derive(Default, Serialize)]
pub struct ParserModel {
    /// Constructs inner `Language` the state to evaluate the version-dependent branches.
    referenced_versions: BTreeSet<Version>,

    /// Defines the `NonterminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Identifier>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<Identifier>,
    /// Defines `EdgeLabel` enum variants.
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

/// Collects model state from the DSL v2 rather than via the remaining visitor pattern.
struct DslV2CollectorState {
    /// Constructs inner `Language` the state to evaluate the version-dependent branches.
    referenced_versions: BTreeSet<Version>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Identifier>,
    /// Defines the `NonterminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<Identifier>,
    /// Defines `EdgeLabel` enum variants.
    labels: BTreeSet<String>,
}

impl ParserModel {
    pub fn from_language(language: &Rc<Language>) -> Self {
        // First, we construct the DSLv1 model from the DSLv2 definition...
        let grammar = Grammar::from_dsl_v2(language);
        // ...which we then transform into the parser model
        let mut acc = ParserAccumulatorState::default();
        grammar.accept_visitor(&mut acc);

        // WIP(#638): Gradually migrate off `GrammarVisitor`
        let terminal_kinds = language
            .items()
            .filter(|item| item.is_terminal() && !matches!(item, Item::Fragment { .. }))
            .map(|item| item.name().clone())
            .collect();

        let mut nonterminal_kinds = BTreeSet::default();
        for item in language.items() {
            match item {
                Item::Struct { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Enum { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Repeated { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Separated { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Precedence { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                    for op in &item.precedence_expressions {
                        nonterminal_kinds.insert(op.name.clone());
                    }
                }
                // Terminals
                _ => {}
            }
        }

        let trivia_scanner_names = language
            .items()
            .filter_map(|item| match item {
                Item::Trivia { item } => Some(item.name.clone()),
                _ => None,
            })
            .collect();

        let mut labels = BTreeSet::default();
        for item in language.items() {
            match item {
                Item::Struct { item } => {
                    for field_name in item.fields.keys() {
                        labels.insert(field_name.to_string());
                    }
                }
                Item::Precedence { item } => {
                    for item in &item.precedence_expressions {
                        for item in &item.operators {
                            for field_name in item.fields.keys() {
                                labels.insert(field_name.to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        acc.into_model(DslV2CollectorState {
            referenced_versions: language.collect_breaking_versions(),
            terminal_kinds,
            nonterminal_kinds,
            trivia_scanner_names,
            labels,
        })
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

    fn into_model(self, collected: DslV2CollectorState) -> ParserModel {
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

        ParserModel {
            parser_functions: self.parser_functions,
            trivia_parser_functions: self.trivia_parser_functions,
            // These are derived from the accumulated state
            scanner_contexts: contexts,
            scanner_functions,
            keyword_compound_scanners,
            // These are derived from the DSLv2 model directly
            referenced_versions: collected.referenced_versions,
            terminal_kinds: collected.terminal_kinds,
            nonterminal_kinds: collected.nonterminal_kinds,
            trivia_scanner_names: collected.trivia_scanner_names,
            labels: collected.labels,
        }
    }
}

impl GrammarVisitor for ParserAccumulatorState {
    fn scanner_definition_enter(&mut self, scanner: &ScannerDefinitionRef) {
        self.all_scanners
            .insert(scanner.name().clone(), Rc::clone(scanner));
    }

    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context().clone());

        self.trivia_parser_functions.insert(
            parser.name().clone(),
            RustCode(parser.to_parser_code().to_string()),
        );
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        // Have to set this regardless so that we can collect referenced scanners
        self.set_current_context(parser.context().clone());
        if !parser.is_inline() {
            self.parser_functions.insert(
                parser.name().clone(),
                RustCode(parser.to_parser_code().to_string()),
            );
        }
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context().clone());

        // While it's not common to parse a precedence expression as a standalone nonterminal,
        // we generate a function for completeness.
        for (name, code) in parser.to_precedence_expression_parser_code() {
            self.parser_functions
                .insert(name.clone(), RustCode(code.to_string()));
        }

        self.parser_functions.insert(
            parser.name().clone(),
            RustCode(parser.to_parser_code().to_string()),
        );
    }

    fn parser_definition_node_enter(&mut self, node: &ParserDefinitionNode) {
        match node {
            ParserDefinitionNode::ScannerDefinition(scanner) => {
                self.top_level_scanner_names.insert(scanner.name().clone());

                self.current_context()
                    .scanner_definitions
                    .insert(scanner.name().clone());
            }
            ParserDefinitionNode::KeywordScannerDefinition(scanner) => {
                self.current_context()
                    .keyword_scanner_defs
                    .insert(scanner.name().clone(), Rc::clone(scanner));
            }

            // Collect delimiters for each context
            ParserDefinitionNode::DelimitedBy(open, _, close, ..) => {
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
