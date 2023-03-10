use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

#[derive(Clone, Debug, Default)]
pub struct VersionedFunctionBody {
    pub comment: String,
    pub versions: BTreeMap<Version, TokenStream>,
}

impl VersionedFunctionBody {
    fn insert(&mut self, comment: String, version: &Version, body: TokenStream) {
        self.comment = comment;
        self.versions.insert(version.clone(), body);
    }

    fn to_function_body(&self) -> TokenStream {
        let first_version = self.versions.iter().next().unwrap().0;
        self.versions
            .iter()
            .rev()
            .map(|(version, body)| {
                let version_flag = format_ident!(
                    "version_is_equal_to_or_greater_than_{}",
                    version.to_string().replace(".", "_")
                );
                if version == first_version {
                    quote! { { #body } }
                } else {
                    quote! { if self.#version_flag { #body } }
                }
            })
            .reduce(|a, b| quote! { #a else #b })
            .unwrap()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CodeGenerator {
    pub token_kinds: BTreeMap<String, Option<String>>,
    pub scanners: BTreeMap<String, VersionedFunctionBody>,

    pub rule_kinds: BTreeSet<String>,
    pub parsers: BTreeMap<String, VersionedFunctionBody>,

    pub errors: Vec<String>,
}

impl CodeGenerator {
    pub fn add_token_kind(&mut self, name: String) -> Ident {
        let name = name;
        let ident = format_ident!("{}", name);
        self.token_kinds.insert(name, None);
        ident
    }

    pub fn add_scanner(
        &mut self,
        name: String,
        version: &Version,
        comment: String,
        body: TokenStream,
    ) {
        self.scanners
            .entry(name)
            .or_default()
            .insert(comment, version, body);
    }

    pub fn add_rule_kind(&mut self, name: String) -> Ident {
        let name = name;
        let ident = format_ident!("{}", name);
        self.rule_kinds.insert(name);
        ident
    }

    pub fn add_parser(
        &mut self,
        name: String,
        version: &Version,
        comment: String,
        body: TokenStream,
    ) {
        self.parsers
            .entry(name)
            .or_default()
            .insert(comment, version, body);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn version_flag_declarations(&self) -> Vec<String> {
        self.scanners
            .values()
            .chain(self.parsers.values())
            .map(|v| v.versions.keys())
            .flatten()
            .collect::<BTreeSet<_>>()
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!(
                    "version_is_equal_to_or_greater_than_{}",
                    version.replace(".", "_")
                );
                quote! { #[allow(dead_code)] pub(crate) #version_name: bool }.to_string()
            })
            .collect()
    }

    pub fn version_flag_initializers(&self) -> Vec<String> {
        self.scanners
            .values()
            .chain(self.parsers.values())
            .map(|v| v.versions.keys())
            .flatten()
            .collect::<BTreeSet<_>>()
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!(
                    "version_is_equal_to_or_greater_than_{}",
                    version.replace(".", "_")
                );
                quote! { #version_name: Version::parse(#version).unwrap() <= version }.to_string()
            })
            .collect()
    }

    pub fn scanner_functions(&self) -> Vec<String> {
        self.scanners
            .iter()
            .map(|(name, scanner)| {
                let function_name = format_ident!("scan_{}", name.to_snake_case());
                let body = scanner.to_function_body();
                format!(
                    "{}\n{}",
                    scanner.comment,
                    quote! {
                        #[allow(unused_assignments, unused_parens)]
                        pub(crate) fn #function_name(&self, stream: &mut Stream) -> bool {
                            #body
                        }
                    }
                )
            })
            .collect()
    }

    pub fn parser_functions(&self) -> Vec<String> {
        self.parsers
            .iter()
            .map(|(name, parser)| {
                let kind = format_ident!("{}", name);
                let function_name = format_ident!("parse_{}", name.to_snake_case());
                let body = parser.to_function_body();
                format!(
                    "{}\n{}",
                    parser.comment,
                    quote! {
                        #[allow(unused_assignments, unused_parens)]
                        pub(crate) fn #function_name(&self, stream: &mut Stream) -> ParseResult {
                            match #body {
                        Pass{ node, error } => Pass{ node: cst::Node::top_level_rule(RuleKind::#kind, node), error },
                                fail => fail
                            }
                        }
                    }
                )
            })
            .collect()
    }

    pub fn scanner_invocations(&self) -> Vec<String> {
        self.scanners
            .keys()
            .map(|name| {
                let production_kind = format_ident!("{}", name);
                let function_name = format_ident!("scan_{}", name.to_snake_case());
                let error_message = name;
                quote! {
                    ProductionKind::#production_kind => call_scanner(self, input, Language::#function_name, TokenKind::#production_kind, #error_message)
                }
                .to_string()
            })
            .collect()
    }

    pub fn parser_invocations(&self) -> Vec<String> {
        self.parsers
            .keys()
            .map(|name| {
                let production_kind = format_ident!("{}", name);
                let function_name = format_ident!("parse_{}", name.to_snake_case());
                quote! {
                    ProductionKind::#production_kind => call_parser(self, input, Language::#function_name)
                }
                .to_string()
            })
            .collect()
    }

    pub fn token_kinds(&self) -> Vec<String> {
        self.token_kinds.keys().cloned().collect()
    }

    pub fn rule_kinds(&self) -> Vec<String> {
        self.rule_kinds.iter().cloned().collect()
    }

    pub fn production_kinds(&self) -> Vec<String> {
        let mut result: Vec<_> = self
            .scanners
            .iter()
            .chain(self.parsers.iter())
            .map(|(name, _)| name.clone())
            .collect();
        result.sort();
        result
    }

    pub fn write_common_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) {
        // Rebuild if input files are added/removed
        codegen.track_input_dir(
            &codegen
                .repo_root
                .join("crates/codegen/parser_templates/src"),
        );

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/parser_templates/src/shared/cst.rs"),
                &output_dir.join("cst.rs"),
            )
            .unwrap();

        let scanning_macros = codegen
            .read_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/parser_templates/src/shared/macros.rs"),
            )
            .unwrap();
        {
            let scanner_functions = self.scanner_functions().join("\n\n");
            codegen
                .write_file(
                    &output_dir.join("scanners.rs"),
                    &format!(
                        "
                        use super::language::*;

                        {scanning_macros}
                            
                        impl Language {{
                            {scanner_functions}
                        }}
                        ",
                    ),
                )
                .unwrap();
        }

        {
            let parser_functions = self.parser_functions().join("\n\n");
            let trivia_functions = quote! {
                fn optional_leading_trivia(&self, stream: &mut Stream) -> Option<Rc<cst::Node>> {
                    let save = stream.position();
                    match self.parse_leading_trivia(stream) {
                        Fail{ .. } => {
                            stream.set_position(save);
                            None
                        },
                        Pass{ node, .. } => Some(node),
                    }
                }
                fn optional_trailing_trivia(&self, stream: &mut Stream) -> Option<Rc<cst::Node>> {
                    let save = stream.position();
                    match self.parse_trailing_trivia(stream) {
                        Fail{ .. } => {
                            stream.set_position(save);
                            None
                        },
                        Pass{ node, .. } => Some(node),
                    }
                }
            };
            codegen
                .write_file(
                    &output_dir.join("parsers.rs"),
                    &format!(
                        "
                        use super::language::*;
                        use super::language::ParseResult::*;

                        {scanning_macros}
                            
                        impl Language {{
                            {trivia_functions}
                            {parser_functions}
                        }}
                        ",
                    ),
                )
                .unwrap();
        }
    }
}
