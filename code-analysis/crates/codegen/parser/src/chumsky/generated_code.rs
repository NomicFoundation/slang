use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

use super::{boilerplate, naming, rustfmt};

#[derive(Clone, Debug, Default)]
pub struct GeneratedCode {
    token_kinds: BTreeMap<String, Option<String>>,
    rule_kinds: BTreeSet<String>,
    parsers: BTreeMap<String, Parser>,
    errors: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Parser {
    comment: Vec<String>,
    result_type: TokenStream,
    versions: BTreeMap<Version, TokenStream>,
}

impl GeneratedCode {
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
        result_type: TokenStream,
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

    pub fn write_to_source_files(&self, output_dir: PathBuf) {
        rustfmt::format_and_write_source(
            &output_dir.join("mod.rs"),
            boilerplate::mod_head().to_string(),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("lex.rs"),
            format!("{}", boilerplate::lex_head()),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("cst.rs"),
            format!("{}", boilerplate::cst_head()),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("ast.rs"),
            format!("{}", boilerplate::ast_head()),
        );

        let mut versions = BTreeSet::new();
        let mut field_definitions = vec![];
        let mut parser_predeclarations = vec![];
        let mut parser_definitions = vec![];
        let mut field_assignments = vec![];

        for (name, parser) in &self.parsers {
            let parser_name = naming::to_parser_name_ident(&name);
            let field_name = naming::to_field_name_ident(&name);
            let result_type = parser.result_type.clone();

            versions.extend(parser.versions.keys());

            parser_predeclarations.push(quote!( let mut #parser_name = Recursive::<char, #result_type, ErrorType>::declare();  ).to_string());

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
                    if version == &Version::new(0, 0, 0) {
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
                quote!( pub #field_name: ParserType<#result_type>, ).to_string()
            ));

            field_assignments
                .push(quote!( #field_name: #parser_name.then_ignore(end()).boxed(), ).to_string());
        }

        let version_declarations = versions
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!("version_{}", version.replace(".", "_"));
                quote!( let #version_name = Version::parse(#version).unwrap(); ).to_string()
            })
            .collect::<Vec<String>>();

        rustfmt::format_and_write_source(
            &output_dir.join("parse.rs"),
            format!(
                "{}
                
                #[allow(dead_code)]
                pub struct Parsers {{
                    {}
                }}

                impl Parsers {{
                    pub fn new(version: Version) -> Self {{
                        // Declare all versions -----------------------------

                        {}

                        // Declare all productions --------------------------

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
        );

        // Do the kinds last, because the code generation steps above may have added new kinds
        rustfmt::format_and_write_source(
            &output_dir.join("kinds.rs"),
            format!(
                "{}

                #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
                pub enum Token {{
                    {}
                }}

                #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
                pub enum Rule {{
                    {}
                }}
                ",
                boilerplate::kinds_head(),
                self.token_kinds
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(","),
                self.rule_kinds
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        );
    }
}
