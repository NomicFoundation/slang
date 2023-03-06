use quote::quote;
use std::path::PathBuf;

use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_typescript_lib_sources(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        self.write_common_sources(codegen, output_dir);

        codegen
            .write_file(
                &output_dir.join("mod.rs"),
                &quote! {
                   pub mod kinds;
                   pub mod cst;
                   pub mod cst_types;
                   pub mod language;
                   pub mod parser_output;
                   mod scanners;
                   mod parsers;
                }
                .to_string(),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/parser_templates/src/typescript/cst_types.rs"),
                &output_dir.join("cst_types.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/parser_templates/src/typescript/parser_output.rs"),
                &output_dir.join("parser_output.rs"),
            )
            .unwrap();

        {
            let language_boilerplate_common = codegen
                .read_file(
                    &codegen
                        .repo_root
                        .join("crates/codegen/parser_templates/src/shared/language.rs"),
                )
                .unwrap();
            let version_flag_declarations = self.version_flag_declarations().join(",");
            let version_flag_initializers = self.version_flag_initializers().join(",");
            let scanner_invocations = self.scanner_invocations().join(",");
            let parser_invocations = self.parser_invocations().join(",");
            let versions_array = {
                let versions = grammar.versions.iter().map(|v| v.to_string());
                quote! { static VERSIONS: &'static [&'static str] = &[ #(#versions),* ]; }
            };
            let language_name = &grammar.title;

            let trampolines = quote! {
                fn call_scanner<F>(language: &Language, input: String, scanner: F, kind: TokenKind, error_message: &str) -> ParserOutput
                    where F: Fn(&Language, &mut Stream) -> bool
                {
                    let mut stream = Stream::new(input.as_str());
                    if scanner(language, &mut stream) && stream.peek().is_none() {
                        ParserOutput { parse_tree: Some(cst::Node::token(kind, Range { start: 0, end: stream.position() }, None, None)), errors: vec![] }
                    } else {
                        ParserOutput { parse_tree: None, errors: vec![ParseError::new(stream.position(), error_message)] }
                    }
                }
                fn call_parser<F>(language: &Language, input: String, parser: F) -> ParserOutput
                    where F: Fn(&Language, &mut Stream) -> ParseResult
                {
                    let mut stream = Stream::new(input.as_str());
                    match parser(language, &mut stream) {
                        ParseResult::Pass{ node, .. } if stream.peek().is_none() => ParserOutput { parse_tree: Some(node), errors: vec![] },
                        ParseResult::Pass{ .. } => ParserOutput { parse_tree: None, errors: vec![ParseError::new(stream.position(), "end of input")] },
                        ParseResult::Fail{ error } => ParserOutput { parse_tree: None, errors: vec![error] }
                    }
                }
            };

            codegen
                .write_file(
                    &output_dir.join("language.rs"),
                    &format!(
                        "
                        {language_boilerplate_common}
                        pub use super::parser_output::ParserOutput;

                        #[napi]
                        pub struct Language {{
                            version: Version,
                            {version_flag_declarations}
                        }}

                        #[napi]
                        impl Language {{
                            #[napi(constructor)]
                            pub fn new(version: String) -> Self {{
                                {versions_array}
                                let version = Version::parse(&version).unwrap();
                                if VERSIONS.contains(&version.to_string().as_str()) {{
                                    Self {{
                                        {version_flag_initializers},
                                        version,
                                    }}
                                }} else {{
                                    panic!(\"Invalid {language_name} language version: {{}}\", version);
                                }}
                            }}

                            #[napi]
                            pub fn version(&self) -> String {{
                                self.version.to_string()
                            }}

                            #[napi]
                            pub fn parse(&self, kind: ProductionKind, input: String) -> ParserOutput {{
                                {trampolines}
                                match kind {{
                                    {scanner_invocations},
                                    {parser_invocations},
                                }}
                            }}
                        }}
                        "
                    ),
                )
                .unwrap();
        }

        {
            // Do the kinds last, because the code generation steps above may have added new kinds
            let kinds_boilerplate_rust = quote! {
               use serde::Serialize;
               use napi::bindgen_prelude::*;
               use napi_derive::napi;
            };
            let token_kinds = self.token_kinds().join(",");
            let rule_kinds = self.rule_kinds().join(",");
            let production_kinds = self.production_kinds().join(",");

            codegen
                .write_file(
                    &output_dir.join("kinds.rs"),
                    &format!(
                        "
                        {kinds_boilerplate_rust}

                        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                        #[napi]
                        pub enum TokenKind {{
                            {token_kinds}
                        }}

                        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                        #[napi]
                        pub enum RuleKind {{
                            {rule_kinds}
                        }}

                        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                        #[napi]
                        pub enum ProductionKind {{
                            {production_kinds},
                        }}
                        ",
                    ),
                )
                .unwrap();
        }
    }
}
