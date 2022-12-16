use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

use super::{boilerplate, naming};

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
        let kind = name.to_pascal_case();
        let ident = format_ident!("{}", kind);
        self.rule_kinds.insert(kind);
        ident
    }

    pub fn add_token_kind(&mut self, name: String) -> Ident {
        let kind = name.to_pascal_case();
        let ident = format_ident!("{}", kind);
        self.token_kinds.insert(kind, None);
        ident
    }

    pub fn add_terminal_kind(&mut self, terminal: String) -> Ident {
        let kind = naming::name_of_terminal_string(&terminal).to_pascal_case();
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
        let mut field_definitions = vec![];
        let mut parser_predeclarations = vec![];
        let mut parser_definitions = vec![];
        let mut field_assignments = vec![];

        let first_version = grammar.manifest.versions.first().unwrap();

        for (name, parser) in &self.parsers {
            let parser_name = naming::to_parser_name_ident(&name);
            let field_name = naming::to_field_name_ident(&name);

            let result_type = match parser.result_type {
                ParserResultType::Token => quote! { DeferredLexParserType },
                ParserResultType::Rule => quote! { DeferredCSTParserType },
            };

            versions.extend(parser.versions.keys());

            parser_predeclarations
                .push(quote!( let mut #parser_name = #result_type::declare(); ).to_string());

            parser_definitions.push(format!(
                "{}\n{}",
                parser
                    .comment
                    .iter()
                    .map(|s| format!("// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n"),
                parser.versions.iter().rev().map(|(version, body)| {
                    let version_name = format_ident!("version_{}", version.to_string().replace(".", "_"));
                    if version == first_version {
                        quote!( { #parser_name.define(#body.boxed()); } )
                    } else {
                        quote!( if #version_name <= version { #parser_name.define(#body.boxed()); } )
                    }
                }).reduce(|a, b| quote!( #a else #b )).unwrap()
            ));

            field_definitions.push(format!(
                "{}\n{}",
                parser
                    .comment
                    .iter()
                    .map(|s| format!("/// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n"),
                quote!( pub #field_name: BoxedParserType, ).to_string()
            ));

            let parser = match parser.result_type {
                ParserResultType::Token => {
                    quote!( #parser_name.map(|node| cst::Node::top_level_token(node)) )
                }
                ParserResultType::Rule => {
                    let kind = format_ident!("{}", name.to_pascal_case());
                    quote!( #parser_name.map(|node| cst::Node::top_level_rule(kinds::Rule::#kind, node)) )
                }
            };
            field_assignments
                .push(quote!( #field_name: #parser.then_ignore(end()).boxed(), ).to_string());
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
                
                    #[allow(dead_code)]
                    pub struct Parsers {{
                        {}
                    }}

                    impl Parsers {{
                        pub fn new(version: &Version) -> Self {{
                            // Declare all versions -----------------------------

                            {}

                            // Declare all productions --------------------------

                            type DeferredLexParserType = Recursive<'static, char, Option<Rc<lex::Node>>, ErrorType>;
                            type DeferredCSTParserType = Recursive<'static, char, Option<Rc<cst::Node>>, ErrorType>;

                            {}

                            // Macros -------------------------------------------

                            {}

                            // Define all productions ---------------------------

                            {}

                            // Create the Parser object -------------------------

                            Self {{
                                {}
                            }}
                        }}
                    }}
                    ",
                    boilerplate::parse_head(),
                    field_definitions.join("\n\n"),
                    version_declarations.join(""),
                    parser_predeclarations.join(""),
                    boilerplate::parse_macros(),
                    parser_definitions.join("\n\n"),
                    field_assignments.join("")
                ),
            )
            .unwrap();
    }
}
