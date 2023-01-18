use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use codegen_schema::grammar::Grammar;
use codegen_utils::context::CodegenContext;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

use super::{boilerplate, naming, to_lexer_code, to_parser_code, to_scanner_code};

#[derive(Clone, Debug, Default)]
pub struct CodeGenerator {
    pub token_kinds: BTreeMap<String, Option<String>>,
    pub rule_kinds: BTreeSet<String>,
    pub parsers: BTreeMap<String, GeneratedParser>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub enum ParserResultType {
    #[default]
    Token,
    PrecedenceRuleMember,
    Rule,
}

#[derive(Clone, Debug, Default)]
pub struct GeneratedParser {
    pub comment: Vec<String>,
    pub result_type: ParserResultType,
    pub versions: BTreeMap<Version, TokenStream>,
}

impl CodeGenerator {
    pub fn add_rule_kind(&mut self, name: String) -> Ident {
        let name = name;
        let ident = format_ident!("{}", name);
        self.rule_kinds.insert(name);
        ident
    }

    pub fn add_token_kind(&mut self, name: String) -> Ident {
        let name = name;
        let ident = format_ident!("{}", name);
        self.token_kinds.insert(name, None);
        ident
    }

    pub fn add_terminal_kind(&mut self, terminal: String) -> Ident {
        let kind = naming::name_of_terminal_string(&terminal);
        let ident = format_ident!("{}", kind);
        self.token_kinds.insert(kind, Some(terminal));
        ident
    }

    pub fn add_parser(
        &mut self,
        name: String,
        version: &Version,
        comment: Vec<String>,
        body: TokenStream,
        result_type: ParserResultType,
    ) {
        // TODO: assert consistency of return types across all versions of the same
        // parser

        let mut entry = self.parsers.entry(name).or_default();
        entry.versions.insert(version.clone(), body);
        entry.result_type = result_type;
        entry.comment = comment;
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn write_parser_source(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        let mut versions = BTreeSet::new();
        let mut parser_predeclarations = vec![];
        let mut parser_definitions = vec![];

        let first_version = grammar.manifest.versions.first().unwrap();

        for (name, parser) in &self.parsers {
            let production_kind = format_ident!("{}", name);

            versions.extend(parser.versions.keys());

            parser_predeclarations.push(
                match parser.result_type {
                    ParserResultType::Token => quote! { declare_token!(#production_kind); },
                    ParserResultType::Rule => quote! { declare_rule!(#production_kind); },
                    ParserResultType::PrecedenceRuleMember => {
                        quote! { declare_rule!(#production_kind); }
                    }
                }
                .to_string(),
            );

            let parser_body = parser
                .versions
                .iter()
                .rev()
                .map(|(version, body)| {
                    let version_name =
                        format_ident!("version_{}", version.to_string().replace(".", "_"));
                    if version == first_version {
                        if parser.versions.len() == 1 {
                            body.clone()
                        } else {
                            quote!( { #body.boxed() } )
                        }
                    } else {
                        quote!( if #version_name <= version { #body.boxed() } )
                    }
                })
                .reduce(|a, b| quote!( #a else #b ))
                .unwrap();
            let production_kind = format_ident!("{}", name);
            parser_definitions.push(format!(
                "{}\n{}",
                parser
                    .comment
                    .iter()
                    .map(|s| format!("// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n"),
                match parser.result_type {
                    ParserResultType::Token =>
                        quote! { define_token!(#production_kind, #parser_body); },
                    ParserResultType::PrecedenceRuleMember =>
                        quote! { define_precedence_rule_member!(#production_kind, #parser_body); },
                    ParserResultType::Rule =>
                        quote! { define_rule!(#production_kind, #parser_body); },
                }
            ));
        }

        let version_declarations = versions
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!("version_{}", version.replace(".", "_"));
                quote!( let #version_name = &Version::parse(#version).unwrap(); ).to_string()
            })
            .collect::<Vec<String>>();

        codegen
            .write_file(
                &output_dir.join("parse.rs"),
                &format!(
                    "{}

                    pub fn create_parsers(version: &Version) -> BTreeMap<ProductionKind, Parser> {{
                        let mut parsers: BTreeMap<ProductionKind, Parser> = BTreeMap::new();
                        
                        // Declare all versions -----------------------------

                        {}

                        // Declare all productions --------------------------

                        {}

                        // Macros -------------------------------------------

                        {}
                        {}
                        {}

                        // Define all productions ---------------------------

                        {}

                        // Return the Parsers object ------------------------

                        parsers
                    }}
                    ",
                    boilerplate::parse_head(),
                    version_declarations.join(""),
                    parser_predeclarations.join(""),
                    to_scanner_code::parse_macros(),
                    to_lexer_code::parse_macros(),
                    to_parser_code::parse_macros(),
                    parser_definitions.join("\n\n"),
                ),
            )
            .unwrap();
    }
}
