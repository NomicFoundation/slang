use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use codegen_schema::types::schema::Schema;
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

#[derive(Clone, Debug)]
pub struct VersionedFunctionBody(BTreeMap<Version, Option<(String, TokenStream)>>);

impl VersionedFunctionBody {
    fn new(first_schema_version: &Version) -> Self {
        let mut versions = BTreeMap::new();
        versions.insert(first_schema_version.clone(), None);
        Self(versions)
    }

    fn insert(&mut self, version: &Version, definition: Option<(String, TokenStream)>) {
        self.0.insert(version.clone(), definition);
    }

    fn is_defined_for_all_versions(&self) -> bool {
        self.0.values().all(|f| f.is_some())
    }

    fn to_function_body(&self, name: &Ident, return_type: TokenStream) -> (String, Ident) {
        let mut per_version_functions = self.0.iter().filter_map(|(version, value)| {
            value.as_ref().map(|(comment, body)| {
                let version_name = version.to_string().replace(".", "_");
                let per_version_function_name = format_ident!( "{name}_{version_name}",);
                let function = quote! {
                    #[allow(unused_assignments, unused_parens)]
                    fn #per_version_function_name(&self, stream: &mut Stream) -> #return_type { #body }
                };
                format!("{comment}\n{function}")
            })
        }).collect::<Vec<_>>();

        let function_name = if self.is_defined_for_all_versions() {
            if self.0.len() == 1 {
                let version = self.0.keys().next().unwrap();
                let version_name = version.to_string().replace(".", "_");
                format_ident!("{name}_{version_name}")
            } else {
                let dispatch_function_name = format_ident!("dispatch_{name}");
                let dispatch_function = {
                    let first_version = self.0.keys().next().unwrap();
                    let body = self
                        .0
                        .iter()
                        .rev()
                        .map(|(version, _)| {
                            let version_name = version.to_string().replace(".", "_");
                            let body = {
                                let per_version_function_name =
                                    format_ident!("{name}_{version_name}");
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
                    quote! { fn #dispatch_function_name(&self, stream: &mut Stream) -> #return_type { #body } }
                };
                per_version_functions.push(dispatch_function.to_string());
                dispatch_function_name
            }
        } else {
            let dispatch_function_name = format_ident!("dispatch_{name}");
            let dispatch_function = {
                let first_version = self.0.keys().next().unwrap();
                let body = self
                    .0
                    .iter()
                    .rev()
                    .map(|(version, value)| {
                        let version_name = version.to_string().replace(".", "_");
                        let body = match value {
                            Some(_) => {
                                let per_version_function_name =
                                    format_ident!("{name}_{version_name}");
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
                quote! { fn #dispatch_function_name(&self, stream: &mut Stream) -> Option<#return_type> { #body } }
            };
            per_version_functions.push(dispatch_function.to_string());
            dispatch_function_name
        };

        (per_version_functions.join("\n\n"), function_name)
    }
}

#[derive(Clone, Debug)]
pub struct CodeGenerator {
    pub first_version: Version,

    pub token_kinds: BTreeMap<String, Option<String>>,
    pub scanners: BTreeMap<String, VersionedFunctionBody>,

    pub rule_kinds: BTreeSet<String>,
    pub parsers: BTreeMap<String, VersionedFunctionBody>,

    pub errors: Vec<String>,
}

impl CodeGenerator {
    pub fn new(schema: &Schema) -> Self {
        Self {
            first_version: schema.versions.first().unwrap().clone(),
            token_kinds: Default::default(),
            scanners: Default::default(),
            rule_kinds: Default::default(),
            parsers: Default::default(),
            errors: Default::default(),
        }
    }

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
        definition: Option<(String, TokenStream)>,
    ) {
        self.scanners
            .entry(name)
            .or_insert_with(|| VersionedFunctionBody::new(&self.first_version))
            .insert(version, definition);
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
                let internal_function_name = format_ident!("scan_{name}", name = name.to_snake_case());
                let (functions, dispatch_function_name) = scanner.to_function_body(&internal_function_name, quote! {bool});
                if scanner.is_defined_for_all_versions() {
                    let internal_function = quote! {
                        #[inline]
                        pub(crate) fn #internal_function_name(&self, stream: &mut Stream) -> bool {
                            self.#dispatch_function_name(stream)
                        }
                    };
                    format!("{functions}\n\n{internal_function}")
                } else {
                    let external_function_name = format_ident!("maybe_scan_{name}", name = name.to_snake_case());
                    let external_function = quote! {
                        #[inline]
                        pub(crate) fn #external_function_name(&self, stream: &mut Stream) -> Option<bool> {
                            self.#dispatch_function_name(stream)
                        }
                    };
                    let internal_function = quote! {
                        #[inline]
                        pub(crate) fn #internal_function_name(&self, stream: &mut Stream) -> bool {
                            self.#dispatch_function_name(stream).expect("Validation should have checked that references are valid between versions")
                        }
                    };
                    format!("{functions}\n\n{external_function}\n\n{internal_function}")
                }
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn scanner_invocations(&self) -> TokenStream {
        let invocations = self.scanners.iter().map(|(name, scanner)| {
            let production_kind = format_ident!("{name}");
            let token_kind = format_ident!("{name}");
            if scanner.is_defined_for_all_versions() {
                let function_name = format_ident!("scan_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => call_scanner(self, input, Language::#function_name, TokenKind::#token_kind, #name) }
            } else {
                let function_name = format_ident!("maybe_scan_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => try_call_scanner(self, input, Language::#function_name, TokenKind::#token_kind, #name) }
            }
        });
        quote! { #(#invocations),* }
    }

    pub fn parser_functions(&self) -> String {
        let functions = self
            .parsers
            .iter()
            .map(|(name, parser)| {
                let kind = format_ident!("{name}");
                let internal_function_name = format_ident!("parse_{name}", name = name.to_snake_case());
                let (functions, dispatch_function_name) = parser.to_function_body(&internal_function_name, quote! {ParserResult});
                if parser.is_defined_for_all_versions() {
                    let internal_function = quote! {
                        #[inline]
                        pub(crate) fn #internal_function_name(&self, stream: &mut Stream) -> ParserResult {
                            match self.#dispatch_function_name(stream) {
                                Pass{ node, error } => Pass{ node: cst::Node::top_level_rule(RuleKind::#kind, node), error },
                                fail => fail
                            }
                        }
                    };
                    format!("{functions}\n\n{internal_function}")
                } else {
                    let external_function_name = format_ident!("maybe_parse_{name}", name = name.to_snake_case());
                    let external_function = quote! {
                        pub(crate) fn #external_function_name(&self, stream: &mut Stream) -> Option<ParserResult> {
                            self.#dispatch_function_name(stream).map(|body|
                                match body {
                                    Pass{ node, error } => Pass{ node: cst::Node::top_level_rule(RuleKind::#kind, node), error },
                                    fail => fail
                                }
                            )
                        }
                    };
                    let internal_function = quote! {
                        #[inline]
                        pub(crate) fn #internal_function_name(&self, stream: &mut Stream) -> ParserResult {
                            self.#external_function_name(stream).expect("Validation should have checked that references are valid between versions")
                        }
                    };
                    format!("{functions}\n\n{external_function}\n\n{internal_function}")
                }
            })
            .collect::<Vec<_>>();
        functions.join("\n\n")
    }

    pub fn parser_invocations(&self) -> TokenStream {
        let invocations = self.parsers.iter().map(|(name, parser)| {
            let production_kind = format_ident!("{name}");
            if parser.is_defined_for_all_versions() {
                let function_name = format_ident!("parse_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => call_parser(self, input, Language::#function_name) }
            } else {
                let function_name = format_ident!("maybe_parse_{name}", name = name.to_snake_case());
                quote!{ ProductionKind::#production_kind => try_call_parser(self, input, Language::#function_name) }
            }
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
                .join("crates/codegen/syntax_templates/src"),
        );

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/syntax_templates/src/shared/cst.rs"),
                &output_dir.join("cst.rs"),
            )
            .unwrap();

        let scanning_macros = codegen
            .read_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/syntax_templates/src/shared/macros.rs"),
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
                use super::language::ParserResult::*;

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
