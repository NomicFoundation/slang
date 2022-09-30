use std::{collections::HashMap, path::PathBuf};

use inflector::Inflector;
use proc_macro2::Ident;
use quote::format_ident;

use super::{boilerplate, naming, rustfmt};

#[derive(Clone, Debug, Default)]
pub struct CodeFragments {
    kinds: HashMap<Kind, HashMap<String, Option<String>>>,
    fragments: HashMap<CodeLocation, Vec<String>>,
    errors: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CodeLocation {
    ParserFieldDefinition,
    ParserFieldAssignment,
    ParserPredeclaration,
    ParserDefinition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Kind {
    Rule,
    Token,
}

impl CodeFragments {
    pub fn add_rule_kind(&mut self, name: String) -> Ident {
        let kind = name.to_pascal_case();
        let ident = format_ident!("{}", kind);
        self.add_kind(Kind::Rule, kind, None);
        ident
    }

    pub fn add_token_kind(&mut self, name: String) -> Ident {
        let kind = name.to_pascal_case();
        let ident = format_ident!("{}", kind);
        self.add_kind(Kind::Token, kind, None);
        ident
    }

    pub fn add_terminal_kind(&mut self, terminal: String) -> Ident {
        let kind = naming::name_of_terminal_string(&terminal).to_pascal_case();
        let ident = format_ident!("{}", kind);
        self.add_kind(Kind::Token, kind, Some(terminal));
        ident
    }

    pub fn add_parser_field_definition_fragment(&mut self, fragment: String) {
        self.add_fragment(CodeLocation::ParserFieldDefinition, fragment);
    }

    pub fn add_parser_field_assignment_fragment(&mut self, fragment: String) {
        self.add_fragment(CodeLocation::ParserFieldAssignment, fragment);
    }

    pub fn add_parser_predeclaration_fragment(&mut self, fragment: String) {
        self.add_fragment(CodeLocation::ParserPredeclaration, fragment);
    }

    pub fn add_parser_definition_fragment(&mut self, fragment: String) {
        self.add_fragment(CodeLocation::ParserDefinition, fragment);
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

        rustfmt::format_and_write_source(
            &output_dir.join("parse.rs"),
            format!(
                "{}
                
                #[allow(dead_code)]
                pub struct Parsers {{
                    {}
                }}

                impl Parsers {{
                    pub fn new() -> Self {{
                        // Declare all productions -----------------------------

                        {}

                        // Macros ----------------------------------------

                        {}

                        // Define all productions ------------------------------

                        {}

                        // Create the Parser object ----------------------------

                        Self {{
                            {}
                        }}
                    }}
                }}
                ",
                boilerplate::parse_head(),
                self.get_fragments(CodeLocation::ParserFieldDefinition)
                    .join(""),
                self.get_fragments(CodeLocation::ParserPredeclaration)
                    .join(""),
                boilerplate::parse_macros(),
                self.get_fragments(CodeLocation::ParserDefinition).join(""),
                self.get_fragments(CodeLocation::ParserFieldAssignment)
                    .join("")
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
                self.get_kinds(Kind::Token).join(","),
                self.get_kinds(Kind::Rule).join(",")
            ),
        );
    }

    fn add_kind(&mut self, kind: Kind, name: String, content: Option<String>) {
        let m = self.kinds.entry(kind).or_default();
        if m.contains_key(&name) {
            if kind == Kind::Token {
                if content != m[&name] {
                    if content.is_none() {
                        self.errors
                            .push(format!("Token {} is defined multiple times", name));
                    } else {
                        self.errors.push(format!(
                            "Token {} is defined multiple times with different content: {:?} vs {:?}",
                            name,
                            content,
                            m[&name].clone()
                        ));
                    }
                }
            } else {
                self.errors
                    .push(format!("{:?} {} is defined multiple times", kind, name));
            }
        } else {
            m.insert(name, content);
        }
    }

    fn add_fragment(&mut self, location: CodeLocation, code: String) {
        self.fragments.entry(location).or_default().push(code);
    }

    fn get_kinds(&self, kind: Kind) -> Vec<String> {
        self.kinds
            .get(&kind)
            .map(|m| {
                let mut k: Vec<String> = m.keys().cloned().collect();
                k.sort();
                k
            })
            .unwrap_or_default()
    }

    fn get_fragments(&self, location: CodeLocation) -> Vec<String> {
        self.fragments.get(&location).cloned().unwrap_or_default()
    }
}
