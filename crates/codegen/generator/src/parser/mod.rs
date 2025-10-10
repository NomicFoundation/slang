//! Defines parser code generation for the language grammar.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use language_definition::model::{self, Identifier, Language};
use serde::Serialize;

mod codegen;
mod grammar;

use codegen::{
    KeywordScannerDefinitionCodegen as _, ParserDefinitionCodegen as _,
    PrecedenceParserDefinitionCodegen as _, Trie,
};
use grammar::{
    GrammarVisitor, ParserDefinitionNode, ParserDefinitionRef, PrecedenceParserDefinitionRef,
};

use crate::parser::codegen::KeywordItemAtom;
use crate::parser::grammar::resolver::Resolution;
use crate::parser::grammar::{GrammarElement, ResolveCtx, TriviaParserDefinitionRef};

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Default, Clone)]
struct RustCode(String);

/// The main model for the parser + lexer code generation.
#[derive(Default, Serialize)]
pub struct ParserModel {
    /// Defines the top-level scanner functions in `Language`.
    scanner_functions: BTreeMap<Identifier, RustCode>,
    // Defines the `Lexer::next_terminal` method.
    scanner_contexts: BTreeMap<Identifier, ScannerContextModel>,
    /// Defines the top-level compound scanners used when lexing in `Language`.
    keyword_compound_scanners: BTreeMap<Identifier, RustCode>,

    /// Defines the top-level parser functions in `Language`.
    parser_functions: BTreeMap<Identifier, RustCode>,
    /// Defines the top-level trivia parser functions in `Language`.
    trivia_parser_functions: BTreeMap<Identifier, RustCode>,
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

impl ParserModel {
    pub fn from_language(language: &Language) -> Self {
        // First, resolve the grammar structure from the flat list of items
        let resolved = ResolveCtx::resolve(language);

        // Collect all parser functions
        let parser_fns = ParserFunctions::collect(&resolved);
        // Collect all scanner contexts and their scanners
        let mut acc = ScannerContextCollector::default();
        resolved.to_grammar().accept_visitor(&mut acc);
        let lexer_model = acc.into_model(&resolved);

        // Combine everything into the final model
        ParserModel {
            scanner_functions: lexer_model.scanner_functions,
            scanner_contexts: lexer_model.scanner_contexts,
            keyword_compound_scanners: lexer_model.keyword_compound_scanners,
            parser_functions: parser_fns.parser_functions,
            trivia_parser_functions: parser_fns.trivia_parser_functions,
        }
    }
}

#[derive(Default)]
struct ScannerContextCollector {
    scanner_contexts: BTreeMap<Identifier, ScannerContextCollectorState>,
    /// Makes sure to codegen the scanner functions that are referenced by other scanners.
    top_level_scanner_names: BTreeSet<Identifier>,
    /// The current context of a parent scanner/parser being processed.
    current_context_name: Option<Identifier>,
}

#[derive(Default)]
struct ScannerContextCollectorState {
    delimiters: BTreeMap<Identifier, Identifier>,
    scanner_definitions: BTreeSet<Identifier>,
    keyword_scanner_defs: BTreeMap<Identifier, Rc<model::KeywordItem>>,
}

impl ScannerContextCollector {
    fn set_current_context(&mut self, name: Identifier) {
        self.current_context_name = Some(name.clone());
        self.scanner_contexts.entry(name).or_default();
    }

    fn current_context(&mut self) -> &mut ScannerContextCollectorState {
        self.scanner_contexts
            .get_mut(self.current_context_name.as_ref().unwrap())
            .expect("context must be set with `set_current_context`")
    }

    // Transforms the accumulated state into the final model.
    fn into_model(mut self, resolved: &Resolution) -> LexerModel {
        // Lookup table for all scanners; used to generate trie scanners.
        let all_scanners: BTreeMap<_, _> = resolved
            .items()
            .filter_map(|(_, item)| item.try_as_scanner_definition_ref())
            .map(|scanner| (scanner.name().clone(), Rc::clone(scanner)))
            .collect();

        for kw_scanner_def in resolved
            .items()
            .filter_map(|(_, item)| item.try_as_keyword_scanner_definition_ref())
        {
            let lex_ctxt = resolved.lex_ctx(&kw_scanner_def.name);

            self.scanner_contexts
                .entry(lex_ctxt.clone())
                .or_default()
                .keyword_scanner_defs
                .insert(kw_scanner_def.name.clone(), Rc::clone(kw_scanner_def));
        }

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
                    let scanner = &all_scanners[scanner_name];

                    let literals = scanner.literals().unwrap_or_default();
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
                    .map(|def| def.identifier.clone())
                    .collect();

                let mut keyword_trie = Trie::new();
                for (name, def) in &context.keyword_scanner_defs {
                    match KeywordItemAtom::try_from_def(def) {
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
        let scanner_functions = all_scanners
            .iter()
            .filter(|(name, scanner)| {
                // are compound (do not consist of only literals)
                scanner.literals().is_none() ||
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

        // These are derived from the accumulated state
        LexerModel {
            scanner_contexts: contexts,
            scanner_functions,
            keyword_compound_scanners,
        }
    }
}

impl GrammarVisitor for ScannerContextCollector {
    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context().clone());
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        self.set_current_context(parser.context().clone());
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context().clone());
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
                // In addition to the context a keyword is defined in, we also
                // need to include reachable ones for the current lexical context
                self.current_context()
                    .keyword_scanner_defs
                    .insert(scanner.name.clone(), Rc::clone(scanner));
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
        }
    }
}

/// Represents a final model used for generating lexer/scanner code.
struct LexerModel {
    scanner_functions: BTreeMap<Identifier, RustCode>,
    scanner_contexts: BTreeMap<Identifier, ScannerContextModel>,
    keyword_compound_scanners: BTreeMap<Identifier, RustCode>,
}

/// Collects all parser functions from the resolved grammar.
struct ParserFunctions {
    parser_functions: BTreeMap<Identifier, RustCode>,
    trivia_parser_functions: BTreeMap<Identifier, RustCode>,
}

impl ParserFunctions {
    fn collect(resolved: &Resolution) -> Self {
        let mut parser_functions = BTreeMap::default();
        let mut trivia_parser_functions = BTreeMap::default();

        for (_, item) in resolved.items() {
            match item {
                GrammarElement::TriviaParserDefinition(parser) => {
                    trivia_parser_functions.insert(
                        parser.name().clone(),
                        RustCode(parser.to_parser_code().to_string()),
                    );
                }
                GrammarElement::ParserDefinition(parser) if !parser.is_inline() => {
                    parser_functions.insert(
                        parser.name().clone(),
                        RustCode(parser.to_parser_code().to_string()),
                    );
                }

                GrammarElement::PrecedenceParserDefinition(parser) => {
                    // While it's not common to parse a precedence expression as a standalone nonterminal,
                    // we generate a function for completeness.
                    for (name, code) in parser.to_precedence_expression_parser_code() {
                        parser_functions.insert(name.clone(), RustCode(code.to_string()));
                    }
                    parser_functions.insert(
                        parser.name().clone(),
                        RustCode(parser.to_parser_code().to_string()),
                    );
                }
                _ => {}
            }
        }

        Self {
            parser_functions,
            trivia_parser_functions,
        }
    }
}
