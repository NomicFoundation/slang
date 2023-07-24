use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

#[derive(Clone, Debug)]
pub struct VersionedFunctionBody(BTreeMap<Version, Option<(String, TokenStream)>>);

impl VersionedFunctionBody {
    fn new(first_language_version: &Version) -> Self {
        let mut versions = BTreeMap::new();
        versions.insert(first_language_version.clone(), None);
        Self(versions)
    }

    fn insert(&mut self, version: &Version, definition: Option<(String, TokenStream)>) {
        self.0.insert(version.clone(), definition);
    }

    fn is_monomorphic(&self) -> bool {
        self.0.len() == 1 && self.is_defined_for_all_versions()
    }

    fn is_defined_for_all_versions(&self) -> bool {
        self.0.values().all(|f| f.is_some())
    }

    fn invocation_name(&self, name: Ident) -> Ident {
        if self.is_defined_for_all_versions() {
            name
        } else {
            format_ident!("{name}__sparse_dispatch")
        }
    }

    fn generate(&self, name: Ident, kind: Option<Ident>, return_type: TokenStream) -> String {
        let kind_wrapper = kind.map(|kind| quote! { .with_kind(RuleKind::#kind) });

        if self.is_monomorphic() {
            let (comment, body) = self.0.first_key_value().unwrap().1.as_ref().unwrap();
            let function = quote! {
                #[
                    // TODO: don't generate if it is inlined.
                    allow(dead_code)
                ]
                #[allow(unused_assignments, unused_parens)]
                pub(crate) fn #name(&self, stream: &mut Stream) -> #return_type { #body #kind_wrapper }
            };
            return format!("{comment}\n{function}");
        };

        let mut functions = self.0.iter().filter_map(|(version, value)| {
            value.as_ref().map(|(comment, body)| {
                let version_name = version.to_string().replace(".", "_");
                let per_version_function_name = format_ident!( "{name}__{version_name}",);
                let function = quote! {
                    #[
                        // Inlined scanners are not always inlined in codegen:
                        // https://github.com/NomicFoundation/slang/issues/365
                        allow(dead_code, non_snake_case)
                    ]
                    fn #per_version_function_name(&self, stream: &mut Stream) -> #return_type { #body #kind_wrapper }
                };
                format!("{comment}\n{function}")
            })
        }).collect::<Vec<_>>();

        if self.is_defined_for_all_versions() {
            let first_version = self.0.keys().next().unwrap();
            let body = self
                .0
                .iter()
                .rev()
                .map(|(version, _)| {
                    let version_name = version.to_string().replace(".", "_");
                    let body = {
                        let per_version_function_name = format_ident!("{name}__{version_name}");
                        quote! { self.#per_version_function_name(stream)  }
                    };
                    let version_flag =
                        format_ident!("version_is_equal_to_or_greater_than_{version_name}");
                    if version == first_version {
                        quote! { { #body } }
                    } else {
                        quote! { if self.#version_flag { #body } }
                    }
                })
                .reduce(|a, b| quote! { #a else #b })
                .unwrap();
            functions.push(
                quote! {
                    pub(crate) fn #name(&self, stream: &mut Stream) -> #return_type { #body }
                }
                .to_string(),
            );
        } else {
            let dispatch_function_name = format_ident!("{name}__sparse_dispatch");
            let first_version = self.0.keys().next().unwrap();
            let body = self
                .0
                .iter()
                .rev()
                .map(|(version, value)| {
                    let version_name = version.to_string().replace(".", "_");
                    let body = match value {
                        Some(_) => {
                            let per_version_function_name = format_ident!("{name}__{version_name}");
                            quote! { Some(self.#per_version_function_name(stream))  }
                        }
                        None => quote! { None },
                    };
                    let version_flag =
                        format_ident!("version_is_equal_to_or_greater_than_{version_name}");
                    if version == first_version {
                        quote! { { #body } }
                    } else {
                        quote! { if self.#version_flag { #body } }
                    }
                })
                .reduce(|a, b| quote! { #a else #b })
                .unwrap();
            functions.push(
                quote! {
                    #[allow(non_snake_case)]
                    pub(crate) fn #dispatch_function_name(&self, stream: &mut Stream) -> Option<#return_type> { #body }
                }.to_string());
            functions.push(quote! {
                #[inline]
                pub(crate) fn #name(&self, stream: &mut Stream) -> #return_type {
                    self.#dispatch_function_name(stream).expect("Validation should have checked that references are valid between versions")
                }
            }.to_string());
        };

        return functions.join("\n\n");
    }
}

#[derive(Clone, Debug)]
pub struct CodeGenerator {
    pub language: LanguageDefinitionRef,
    pub first_version: Version,

    pub scanners: BTreeMap<String, VersionedFunctionBody>,
    pub parsers: BTreeMap<String, VersionedFunctionBody>,

    pub errors: Vec<String>,
}

impl CodeGenerator {
    pub fn new(language: &LanguageDefinitionRef) -> Self {
        Self {
            language: language.clone(),
            first_version: language.versions.first().unwrap().clone(),

            scanners: Default::default(),
            parsers: Default::default(),

            errors: Default::default(),
        }
    }

    pub fn add_scanner(
        &mut self,
        name: String,
        version: &Version,
        definition: Option<(String, TokenStream)>,
    ) {
        self.scanners
            .entry(name)
            .or_insert_with(|| VersionedFunctionBody::new(&self.first_version))
            .insert(version, definition);
    }

    pub fn add_parser(
        &mut self,
        name: String,
        version: &Version,
        definition: Option<(String, TokenStream)>,
    ) {
        self.parsers
            .entry(name)
            .or_insert_with(|| VersionedFunctionBody::new(&self.first_version))
            .insert(version, definition);
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
            .map(|v| v.0.keys())
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
                quote! { pub(crate) #version_name: bool }
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
                scanner.generate(
                    format_ident!("{name}", name = name.to_snake_case()),
                    None,
                    quote! {bool},
                )
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn scanner_invocations(&self) -> TokenStream {
        let invocations = self.scanners.iter()
        .filter(|(name, _)| !self.language.productions[*name].inlined)
        .map(|(name, scanner)| {
            let kind = format_ident!("{name}");
            let function_name = scanner.invocation_name(format_ident!("{name}", name = name.to_snake_case()));
            quote!{ ProductionKind::#kind =>Language::#function_name.scan(self, input, TokenKind::#kind) }
        });
        quote! { #(#invocations),* }
    }

    pub fn token_functions(&self) -> TokenStream {
        return quote! {
            fn parse_token_with_trivia<F>(
                &self,
                stream: &mut Stream,
                scanner: F,
                kind: TokenKind,
            ) -> ParserResult
            where
                F: Fn(&Self, &mut Stream) -> bool,
            {
                let mut children = vec![];

                let restore = stream.position();
                if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
                    children.extend(r#match.nodes);
                } else {
                    stream.set_position(restore);
                }

                let start = stream.position();
                if !scanner(self, stream) {
                    stream.set_position(restore);
                    return ParserResult::no_match(vec![kind]);
                }
                let end = stream.position();
                children.push(cst::Node::token(
                    kind,
                    stream.content(start.utf8..end.utf8)
                ));

                let restore = stream.position();
                if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
                    children.extend(r#match.nodes);
                } else {
                    stream.set_position(restore);
                }

                return ParserResult::r#match(children, vec![]);
            }

            fn parse_token<F>(
                &self,
                stream: &mut Stream,
                scanner: F,
                kind: TokenKind,
            ) -> ParserResult
            where
                F: Fn(&Self, &mut Stream) -> bool,
            {
                let start = stream.position();
                if !scanner(self, stream) {
                    stream.set_position(start);
                    return ParserResult::no_match(vec![kind]);
                }
                let end = stream.position();
                return ParserResult::r#match(vec![cst::Node::token(
                    kind,
                    stream.content(start.utf8..end.utf8)
                )], vec![]);
            }
        };
    }

    pub fn parser_functions(&self) -> String {
        let functions = self
            .parsers
            .iter()
            .map(|(name, parser)| {
                parser.generate(
                    format_ident!("{name}", name = name.to_snake_case()),
                    if self.language.productions[name].inlined {
                        None
                    } else {
                        Some(format_ident!("{name}"))
                    },
                    quote! {ParserResult},
                )
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn parser_invocations(&self) -> TokenStream {
        let invocations = self
            .parsers
            .iter()
            .filter(|(name, _)| !self.language.productions[*name].inlined)
            .map(|(name, parser)| {
                let kind = format_ident!("{name}");
                let function_name =
                    parser.invocation_name(format_ident!("{name}", name = name.to_snake_case()));
                quote! { ProductionKind::#kind => Language::#function_name.parse(self, input) }
            });
        quote! { #(#invocations),* }
    }

    pub fn write_common_sources(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) {
        // Rebuild if input files are added/removed
        codegen.track_input_dir(
            &codegen
                .repo_root
                .join("crates/codegen/legacy_syntax_templates/src"),
        );

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/cst.rs"),
                &output_dir.join("cst.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/cursor.rs"),
                &output_dir.join("cursor.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/scanner_macros.rs"),
                &output_dir.join("scanner_macros.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/parser_helpers.rs"),
                &output_dir.join("parser_helpers.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/parse_error.rs"),
                &output_dir.join("parse_error.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/parser_function.rs"),
                &output_dir.join("parser_function.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/parser_result.rs"),
                &output_dir.join("parser_result.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/scanner_function.rs"),
                &output_dir.join("scanner_function.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/stream.rs"),
                &output_dir.join("stream.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/text_index.rs"),
                &output_dir.join("text_index.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/shared/visitor.rs"),
                &output_dir.join("visitor.rs"),
            )
            .unwrap();

        {
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                use super::language::Language;
                use super::stream::*;

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
            // `token_functions` is copied in because it needs to reference trivia parsers from withing the `Language` instance.
            let content = format!(
                "
                use super::cst;
                use super::language::Language;
                use super::parser_helpers::*;
                use super::parser_result::*;
                use super::stream::*;
                
                use crate::syntax::nodes::{{RuleKind, TokenKind}};

                impl Language {{
                    
                    {token_functions}

                    {parser_functions}

                }}
                ",
                token_functions = self.token_functions(),
                parser_functions = self.parser_functions()
            );

            codegen
                .write_file(&output_dir.join("parsers.rs"), &content)
                .unwrap();
        }
    }
}
