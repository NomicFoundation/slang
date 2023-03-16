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
    fn insert(&mut self, comment: &str, version: &Version, body: TokenStream) {
        self.comment.push_str(comment);
        self.versions.insert(version.clone(), body);
    }

    fn to_function_body(&self) -> TokenStream {
        let first_version = self.versions.iter().next().unwrap().0;
        self.versions
            .iter()
            .rev()
            .map(|(version, body)| {
                let version_flag = format_ident!(
                    "version_is_equal_to_or_greater_than_{version}",
                    version = version.to_string().replace(".", "_")
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
        let ident = format_ident!("{name}");
        self.token_kinds.insert(name, None);
        ident
    }

    pub fn add_scanner(
        &mut self,
        name: String,
        version: &Version,
        comment: &str,
        body: TokenStream,
    ) {
        self.scanners
            .entry(name)
            .or_default()
            .insert(comment, version, body);
    }

    pub fn add_rule_kind(&mut self, name: String) -> Ident {
        let name = name;
        let ident = format_ident!("{name}");
        self.rule_kinds.insert(name);
        ident
    }

    pub fn add_parser(
        &mut self,
        name: String,
        version: &Version,
        comment: &str,
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

    fn scanner_and_parser_version_names(&self) -> BTreeSet<String> {
        self.scanners
            .values()
            .chain(self.parsers.values())
            .map(|v| v.versions.keys())
            .flatten()
            .map(|v| v.to_string())
            .collect()
    }

    pub fn version_flag_declarations(&self) -> TokenStream {
        let bindings = self.scanner_and_parser_version_names();
        let declarations = bindings
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!(
                    "version_is_equal_to_or_greater_than_{version}",
                    version = version.replace(".", "_")
                );
                quote! { #[allow(dead_code)] pub(crate) #version_name: bool }
            });
        quote! { #(#declarations),* }
    }

    pub fn version_flag_initializers(&self) -> TokenStream {
        let bindings = self.scanner_and_parser_version_names();
        let initializers = bindings
            .iter()
            .skip(1) // Don't need the first version
            .map(|version| {
                let version = version.to_string();
                let version_name = format_ident!(
                    "version_is_equal_to_or_greater_than_{version}",
                    version = version.replace(".", "_")
                );
                quote! { #version_name: Version::parse(#version).unwrap() <= version }
            });
        quote! { #(#initializers),* }
    }

    pub fn scanner_functions(&self) -> String {
        let functions = self
            .scanners
            .iter()
            .map(|(name, scanner)| {
                let function_name = format_ident!("scan_{name}", name = name.to_snake_case());
                let body = scanner.to_function_body();
                let comment = &scanner.comment;
                let function = quote! {
                    #[allow(unused_assignments, unused_parens)]
                    pub(crate) fn #function_name(&self, stream: &mut Stream) -> bool {
                        #body
                    }
                };
                format!("{comment}\n{function}")
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn scanner_invocations(&self) -> TokenStream {
        let invocations = self
            .scanners
            .keys()
            .map(|name| {
                let production_kind = format_ident!("{name}");
                let function_name = format_ident!("scan_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => call_scanner(self, input, Language::#function_name, TokenKind::#production_kind, #name) }
            });
        quote! { #(#invocations),* }
    }

    pub fn parser_functions(&self) -> String {
        let functions = self.parsers
            .iter()
            .map(|(name, parser)| {
                let kind = format_ident!("{name}");
                let function_name = format_ident!("parse_{name}", name = name.to_snake_case());
                let body = parser.to_function_body();
                let comment = &parser.comment;
                let function = quote! {
                    #[allow(unused_assignments, unused_parens)]
                    pub(crate) fn #function_name(&self, stream: &mut Stream) -> ParseResult {
                        match #body {
                            Pass{ node, error } => Pass{ node: cst::Node::top_level_rule(RuleKind::#kind, node), error },
                            fail => fail
                        }
                    }
                };
                format!("{comment}\n{function}")
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn parser_invocations(&self) -> TokenStream {
        let invocations = self
            .parsers
            .keys()
            .map(|name| {
                let production_kind = format_ident!("{name}");
                let function_name = format_ident!("parse_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => call_parser(self, input, Language::#function_name) }
            });
        quote! { #(#invocations),* }
    }

    pub fn token_kinds(&self) -> TokenStream {
        let kinds = self
            .token_kinds
            .iter()
            .map(|(name, _)| format_ident!("{name}"));
        quote! {
            pub enum TokenKind {
                #(#kinds),*
            }
        }
    }

    pub fn rule_kinds(&self) -> TokenStream {
        let kinds = self.rule_kinds.iter().map(|name| format_ident!("{name}"));
        quote! {
            pub enum RuleKind {
                #(#kinds),*
            }
        }
    }

    pub fn production_kinds(&self) -> TokenStream {
        let mut kinds: Vec<_> = self
            .scanners
            .iter()
            .chain(self.parsers.iter())
            .map(|(name, _)| format_ident!("{name}"))
            .collect();
        kinds.sort();
        quote! {
            pub enum ProductionKind {
                #(#kinds),*
            }
        }
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
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                use super::language::*;

                {scanning_macros}
                    
                impl Language {{
                    {scanner_functions}
                }}
                ",
                scanner_functions = self.scanner_functions()
            );

            codegen
                .write_file(&output_dir.join("scanners.rs"), &content)
                .unwrap();
        }

        {
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                use super::language::*;
                use super::language::ParseResult::*;

                {scanning_macros}
                    
                impl Language {{
                    {trivia_functions}
                    {parser_functions}
                }}
                ",
                trivia_functions = quote! {
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
                },
                parser_functions = self.parser_functions()
            );

            codegen
                .write_file(&output_dir.join("parsers.rs"), &content)
                .unwrap();
        }
    }
}
