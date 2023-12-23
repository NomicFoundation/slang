use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use codegen_grammar::{
    Grammar, GrammarVisitor, KeywordScannerDefinitionRef, ParserDefinitionNode,
    ParserDefinitionRef, PrecedenceParserDefinitionRef, ScannerDefinitionNode,
    ScannerDefinitionRef, TriviaParserDefinitionRef,
};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;
use quote::{format_ident, quote};
use semver::Version;
use serde::Serialize;

use crate::ast_model::AstModel;
use crate::keyword_scanner_definition::KeywordScannerDefinitionExtensions;
use crate::parser_definition::ParserDefinitionExtensions;
use crate::precedence_parser_definition::PrecedenceParserDefinitionExtensions;
use crate::scanner_definition::ScannerDefinitionExtensions;
use crate::trie::Trie;

#[derive(Default, Serialize)]
pub struct CodeGenerator {
    referenced_versions: BTreeSet<Version>,

    rule_kinds: BTreeSet<&'static str>,
    token_kinds: BTreeSet<&'static str>,
    trivia_kinds: BTreeSet<&'static str>,

    scanner_functions: BTreeMap<&'static str, String>, // (name of scanner, code)
    scanner_contexts: BTreeMap<&'static str, ScannerContext>,
    // All of the keyword scanners (for now we assume we don't have context-specific keywords)
    keyword_scanners: BTreeMap<&'static str, String>,

    parser_functions: BTreeMap<&'static str, String>, // (name of parser, code)

    #[serde(skip)]
    top_level_scanner_names: BTreeSet<&'static str>,
    #[serde(skip)]
    all_scanners: BTreeMap<&'static str, ScannerDefinitionRef>,
    #[serde(skip)]
    current_context_name: &'static str,
}

#[derive(Default, Serialize)]
struct ScannerContext {
    #[serde(skip)]
    scanner_definitions: BTreeSet<&'static str>,
    literal_scanner: String,
    keyword_scanners: BTreeMap<&'static str, String>,
    compound_scanner_names: Vec<&'static str>,
    delimiters: BTreeMap<&'static str, &'static str>,
}

impl CodeGenerator {
    pub fn write_backend(grammar: &Grammar, ast_model: &AstModel, output_dir: &Path) -> Result<()> {
        let mut code = Self::default();
        grammar.accept_visitor(&mut code);
        let code = &code;

        let runtime_dir =
            CargoWorkspace::locate_source_crate("codegen_parser_runtime")?.join("src");
        let mut codegen = Codegen::read_write(&runtime_dir)?;

        {
            #[derive(Serialize)]
            pub struct Template<'a> {
                pub ast_model: &'a AstModel,
            }
            codegen.render(
                Template { ast_model },
                runtime_dir.join("napi/templates/ast_selectors.rs.jinja2"),
                output_dir.join("napi/napi_ast_selectors.rs"),
            )?;
        }

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

    pub fn write_frontend(ast_model: &AstModel, output_dir: &Path) -> Result<()> {
        let runtime_dir =
            CargoWorkspace::locate_source_crate("codegen_parser_runtime")?.join("src");

        let mut codegen = Codegen::read_write(&runtime_dir)?;

        {
            #[derive(Serialize)]
            pub struct Template<'a> {
                pub ast_model: &'a AstModel,
            }
            codegen.render(
                Template { ast_model },
                runtime_dir.join("napi/templates/ast_types.ts.jinja2"),
                output_dir.join("src/ast/generated/ast_types.ts"),
            )?;
        }

        Ok(())
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

impl GrammarVisitor for CodeGenerator {
    fn grammar_leave(&mut self, _grammar: &Grammar) {
        // Expose the scanner functions that...
        self.scanner_functions = self
            .all_scanners
            .iter()
            .filter(|(name, scanner)| {
                // are compound (does not consist of only literals)
                scanner.literals().is_empty() ||
                // but make sure to also include a scanner that is referenced by other scanners, even if not compound
                !self.top_level_scanner_names.contains(*name)
            })
            .map(|(name, scanner)| (*name, scanner.to_scanner_code().to_string()))
            .collect();

        for context in self.scanner_contexts.values_mut() {
            let mut literal_trie = Trie::new();

            // Dr Hackity McHackerson
            // Identifier at the end so it doesn't grab other things.
            // Not a problem when we switch to a DFA.
            let have_identifier_scanner = context.scanner_definitions.remove("Identifier");

            for scanner_name in &context.scanner_definitions {
                let scanner = &self.all_scanners[*scanner_name];

                let literals = scanner.literals();
                if literals.is_empty() {
                    context.compound_scanner_names.push(scanner_name);
                } else {
                    for literal in literals {
                        literal_trie.insert(&literal, scanner.clone());
                    }
                }
            }

            context.literal_scanner = literal_trie.to_scanner_code().to_string();

            if have_identifier_scanner {
                context.compound_scanner_names.push("Identifier");
            }
        }

        // Collect all of the keyword scanners into a single list to be defined at top-level
        self.keyword_scanners = self
            .scanner_contexts
            .values()
            .flat_map(|context| {
                context
                    .keyword_scanners
                    .iter()
                    .map(|(name, code)| (*name, code.to_string()))
            })
            .collect();

        // Just being anal about tidying up :)
        self.all_scanners.clear();
        self.current_context_name = "";
    }

    fn scanner_definition_enter(&mut self, scanner: &ScannerDefinitionRef) {
        self.all_scanners.insert(scanner.name(), scanner.clone());
    }

    fn keyword_scanner_definition_enter(&mut self, scanner: &KeywordScannerDefinitionRef) {
        for def in scanner.definitions() {
            self.referenced_versions
                .extend(def.enabled.iter().map(|vqr| vqr.from.clone()));
            self.referenced_versions
                .extend(def.reserved.iter().map(|vqr| vqr.from.clone()));
        }
    }

    fn trivia_parser_definition_enter(&mut self, parser: &TriviaParserDefinitionRef) {
        self.set_current_context(parser.context());
        self.rule_kinds.insert(parser.name());
        self.trivia_kinds.insert(parser.name());
        self.parser_functions.insert(
            parser.name(),
            {
                let code = parser.to_parser_code();
                let rule_kind = format_ident!("{}", parser.name());
                quote! { #code.with_kind(RuleKind::#rule_kind) }
            }
            .to_string(),
        );
    }

    fn parser_definition_enter(&mut self, parser: &ParserDefinitionRef) {
        // Have to set this regardless so that we can collect referenced scanners
        self.set_current_context(parser.context());
        if !parser.is_inline() {
            self.rule_kinds.insert(parser.name());
            let code = parser.to_parser_code();
            self.parser_functions.insert(
                parser.name(),
                {
                    let rule_kind = format_ident!("{}", parser.name());
                    quote! { #code.with_kind(RuleKind::#rule_kind) }
                }
                .to_string(),
            );
        }
    }

    fn precedence_parser_definition_enter(&mut self, parser: &PrecedenceParserDefinitionRef) {
        self.set_current_context(parser.context());
        self.rule_kinds.insert(parser.name());
        for (_, name, _) in &parser.node().operators {
            self.rule_kinds.insert(name);
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
                let rule_kind = format_ident!("{}", parser.name());
                quote! { #code.with_kind(RuleKind::#rule_kind) }
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
                self.token_kinds.insert(scanner.name());

                self.current_context()
                    .scanner_definitions
                    .insert(scanner.name());
            }
            ParserDefinitionNode::KeywordScannerDefinition(scanner) => {
                self.token_kinds.insert(scanner.name());

                // Assume we don't have context-specific keywords for now
                self.current_context()
                    .keyword_scanners
                    .insert(scanner.name(), scanner.to_scanner_code().to_string());
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
