use std::collections::{BTreeMap, BTreeSet};

use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use semver::Version;

use super::naming;

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
}
