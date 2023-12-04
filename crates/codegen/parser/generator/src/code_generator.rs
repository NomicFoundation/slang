use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
    path::Path,
};

use anyhow::Result;
use infra_utils::{cargo::CargoWorkspace, codegen::Codegen};
use quote::{format_ident, quote};
use semver::Version;
use serde::Serialize;

use codegen_grammar::{
    Grammar, GrammarVisitor, ParserDefinitionNode, ParserDefinitionRef,
    PrecedenceParserDefinitionNode, PrecedenceParserDefinitionRef, ScannerDefinitionNode,
    ScannerDefinitionRef, TriviaParserDefinitionRef,
};

use super::{
    parser_definition::ParserDefinitionExtensions,
    precedence_parser_definition::PrecedenceParserDefinitionExtensions,
    scanner_definition::ScannerDefinitionExtensions, trie::Trie,
};

#[derive(Default, Serialize)]
pub struct CodeGenerator {
    referenced_versions: BTreeSet<Version>,

    rule_kinds: BTreeSet<&'static str>,
    token_kinds: BTreeSet<&'static str>,
    production_kinds: BTreeSet<&'static str>,
    trivia_kinds: BTreeSet<&'static str>,

    top_level_scanner_names: BTreeSet<&'static str>,
    scanner_functions: Vec<(&'static str, String)>, // (name of scanner, code)
    scanner_contexts: Vec<ScannerContext>,

    parser_functions: Vec<(&'static str, String)>, // (name of parser, code)

    #[serde(skip)]
    scanner_contexts_map: BTreeMap<&'static str, ScannerContext>,
    #[serde(skip)]
    all_scanners: BTreeMap<&'static str, ScannerDefinitionRef>,
    #[serde(skip)]
    current_context_name: &'static str,
}

#[derive(Serialize)]
struct ScannerContext {
    name: &'static str,
    #[serde(skip)]
    scanner_definitions: BTreeSet<&'static str>,
    alpha_literal_scanner: String,
    non_alpha_literal_scanner: String,
    compound_scanner_names: Vec<&'static str>,
    delimiters: BTreeMap<&'static str, &'static str>,
}

impl CodeGenerator {
    pub fn write_source(output_dir: &Path, grammar: &Grammar) -> Result<()> {
        let mut code = Self::default();
        grammar.accept_visitor(&mut code);
        let code = &code;

        let runtime_dir =
            CargoWorkspace::locate_source_crate("codegen_parser_runtime")?.join("src");
        let mut codegen = Codegen::read_write(&runtime_dir)?;

        {
            #[derive(Serialize)]
            pub struct Template<'a> {
                pub code: &'a CodeGenerator,
            }
            codegen.render(
                Template { code },
                runtime_dir.join("templates/kinds.rs.jinja2"),
                output_dir.join("kinds.rs"),
            )?;
        }

        {
            #[derive(Serialize)]
            pub struct Template<'a> {
                pub code: &'a CodeGenerator,
                pub language_name: String,
                pub versions: BTreeSet<Version>,
            }
            codegen.render(
                Template {
                    code,
                    language_name: grammar.name.clone(),
                    versions: grammar.versions.clone(),
                },
                runtime_dir.join("templates/language.rs.jinja2"),
                output_dir.join("language.rs"),
            )?;
        }

        {
            #[derive(Serialize)]
            pub struct Template {}
            codegen.render(
                Template {},
                runtime_dir.join("templates/mod.rs.jinja2"),
                output_dir.join("mod.rs"),
            )?;
        }

        for file in &[
            "cst.rs",
            "cursor.rs",
            "lexer.rs",
            "parse_error.rs",
            "parse_output.rs",
            "text_index.rs",
            "napi/napi_cst.rs",
            "napi/napi_cursor.rs",
            "napi/napi_parse_error.rs",
            "napi/napi_parse_output.rs",
            "napi/napi_text_index.rs",
            "napi/mod.rs",
            "support/mod.rs",
            "support/context.rs",
            "support/parser_function.rs",
            "support/optional_helper.rs",
            "support/sequence_helper.rs",
            "support/repetition_helper.rs",
            "support/choice_helper.rs",
            "support/precedence_helper.rs",
            "support/parser_result.rs",
            "support/recovery.rs",
            "support/separated_helper.rs",
            "support/scanner_macros.rs",
        ] {
            codegen.copy_file(runtime_dir.join(file), output_dir.join(file))?;
        }

        Ok(())
    }

    fn set_current_context(&mut self, name: &'static str) {
        self.current_context_name = name;
        self.scanner_contexts_map
            .entry(name)
            .or_insert_with(|| ScannerContext {
                name,
                scanner_definitions: BTreeSet::default(),
                alpha_literal_scanner: "".to_string(),
                non_alpha_literal_scanner: "".to_string(),
                compound_scanner_names: vec![],
                delimiters: BTreeMap::default(),
            });
    }
}

impl GrammarVisitor for CodeGenerator {
    fn grammar_leave(&mut self, _grammar: &Grammar) {
        self.scanner_functions = self
            .all_scanners
            .iter()
            .filter(|(name, scanner)| {
                !self.top_level_scanner_names.contains(*name) || scanner.literals().is_empty()
            })
            .map(|(name, scanner)| (*name, scanner.to_scanner_code().to_string()))
            .collect();

        self.parser_functions.sort_by(|a, b| a.0.cmp(b.0));
        self.scanner_functions.sort_by(|a, b| a.0.cmp(b.0));

        for context in self.scanner_contexts_map.values_mut() {
            let mut alpha_literal_trie = Trie::new();
            let mut non_alpha_literal_trie = Trie::new();
            let mut have_identifier_scanner = false;
            for scanner_name in &context.scanner_definitions {
                let scanner = &self.all_scanners[*scanner_name];
                let literals = scanner.literals();
                if literals.is_empty() {
                    // Dr Hackity McHackerson
                    // Identifier at the end so it doesn't grab other things.
                    // Not a problem when we switch to a DFA.
                    if scanner_name == &"Identifier" {
                        have_identifier_scanner = true;
                    } else {
                        context.compound_scanner_names.push(scanner_name);
                    }
                } else {
                    for literal in literals {
                        // This is good enough until we switch to a DFA
                        if literal.chars().next().unwrap().is_alphabetic() {
                            alpha_literal_trie.insert(literal.as_str(), scanner.clone());
                        } else {
                            non_alpha_literal_trie.insert(literal.as_str(), scanner.clone());
                        }
                    }
                }
            }
            context.alpha_literal_scanner = alpha_literal_trie.to_scanner_code().to_string();
            context.non_alpha_literal_scanner =
                non_alpha_literal_trie.to_scanner_code().to_string();
            if have_identifier_scanner {
                context.compound_scanner_names.push("Identifier");
            }
        }

        self.scanner_contexts = mem::take(&mut self.scanner_contexts_map)
            .into_values()
            .collect();

        // Just being anal about tidying up :)
        self.all_scanners.clear();
        self.current_context_name = "";
    }

    fn scanner_definition_enter(&mut self, scanner: &ScannerDefinitionRef) {
        self.all_scanners.insert(scanner.name(), scanner.clone());
    }

    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context());
        self.production_kinds.insert(parser.name());
        self.rule_kinds.insert(parser.name());
        self.trivia_kinds.insert(parser.name());
        self.parser_functions.push((
            parser.name(),
            {
                let code = parser.to_parser_code();
                let rule_kind = format_ident!("{}", parser.name());
                quote! { #code.with_kind(RuleKind::#rule_kind) }
            }
            .to_string(),
        ))
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        // Have to set this regardless so that we can collect referenced scanners
        self.set_current_context(parser.context());
        if !parser.is_inline() {
            self.production_kinds.insert(parser.name());
            self.rule_kinds.insert(parser.name());
            let code = parser.to_parser_code();
            self.parser_functions.push((
                parser.name(),
                {
                    let rule_kind = format_ident!("{}", parser.name());
                    quote! { #code.with_kind(RuleKind::#rule_kind) }
                }
                .to_string(),
            ));
        }
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context());
        self.production_kinds.insert(parser.name());
        self.rule_kinds.insert(parser.name());
        for (_, _, name, _) in &parser.node().operators {
            self.rule_kinds.insert(name);
        }
        self.parser_functions.push((
            parser.name(),
            {
                let code = parser.to_parser_code();
                let rule_kind = format_ident!("{}", parser.name());
                quote! { #code.with_kind(RuleKind::#rule_kind) }
            }
            .to_string(),
        ))
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
                self.token_kinds.insert(scanner.name());
                self.scanner_contexts_map
                    .get_mut(&self.current_context_name)
                    .unwrap()
                    .scanner_definitions
                    .insert(scanner.name());
            }
            // Collect delimiters for each context
            ParserDefinitionNode::DelimitedBy(open, _, close) => {
                let (open, close) = match (open.as_ref(), close.as_ref()) {
                    (
                        ParserDefinitionNode::ScannerDefinition(open, ..),
                        ParserDefinitionNode::ScannerDefinition(close, ..),
                    ) => (open.name(), close.name()),
                    _ => panic!("DelimitedBy must be delimited by scanners"),
                };

                let delimiters = &mut self
                    .scanner_contexts_map
                    .get_mut(&self.current_context_name)
                    .unwrap()
                    .delimiters;

                assert!(
                    delimiters.get(close).is_none(),
                    "Cannot use a closing delimiter as an opening one"
                );
                delimiters.insert(open, close);
            }
            _ => {}
        };
    }

    fn precedence_parser_definition_node_enter(&mut self, node: &PrecedenceParserDefinitionNode) {
        for operator in &node.operators {
            for vqr in &operator.0 {
                self.referenced_versions.insert(vqr.from.clone());
            }
        }
    }
}
