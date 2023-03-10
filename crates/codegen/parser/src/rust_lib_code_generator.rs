use quote::quote;
use std::path::PathBuf;

use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_rust_lib_sources(
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
                   pub mod cst_visitor;
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
                    .join("crates/codegen/parser_templates/src/rust/cst_visitor.rs"),
                &output_dir.join("cst_visitor.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/parser_templates/src/rust/parser_output.rs"),
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

            let trampolines = quote! {
                fn call_scanner<F>(language: &Language, input: &str, scanner: F, kind: TokenKind, error_message: &str) -> ParserOutput
                    where F: Fn(&Language, &mut Stream) -> bool
                {
                    let mut stream = Stream::new(input);
                    if scanner(language, &mut stream) && stream.peek().is_none() {
                        ParserOutput { parse_tree: Some(cst::Node::token(kind, Range { start: 0, end: stream.position() }, None, None)), errors: vec![] }
                    } else {
                        ParserOutput { parse_tree: None, errors: vec![ParseError::new(stream.position(), error_message)] }
                    }
                }
                fn call_parser<F>(language: &Language, input: &str, parser: F) -> ParserOutput
                    where F: Fn(&Language, &mut Stream) -> ParseResult
                {
                    let mut stream = Stream::new(input);
                    match parser(language, &mut stream) {
                        ParseResult::Pass{ node, .. } if stream.peek().is_none() => ParserOutput { parse_tree: Some(node), errors: vec![] },
                        ParseResult::Pass{ .. } => ParserOutput { parse_tree: None, errors: vec![ParseError::new(stream.position(), "end of input")] },
                        ParseResult::Fail{ error } => ParserOutput { parse_tree: None, errors: vec![error] }
                    }
                }
            };
            let language_name = &grammar.title;

            codegen
                .write_file(
                    &output_dir.join("language.rs"),
                    &format!(
                        "
                        {language_boilerplate_common}
                        pub use super::parser_output::ParserOutput;

                        pub struct Language {{
                            version: Version,
                            {version_flag_declarations}
                        }}

                        #[derive(thiserror::Error, Debug)]
                        pub enum Error {{
                            #[error(\"Invalid {language_name} language version '{{0}}'.\")]
                            InvalidLanguageVersion(Version),
                        }}

                        impl Language {{
                            pub fn new(version: Version) -> Result<Self, Error> {{
                                {versions_array}
                                if VERSIONS.contains(&version.to_string().as_str()) {{
                                    Ok(Self {{
                                        {version_flag_initializers},
                                        version,
                                    }})
                                }} else {{
                                    Err(Error::InvalidLanguageVersion(version))
                                }}
                            }}

                            pub fn version(&self) -> &Version {{
                                &self.version
                            }}

                            pub fn parse(&self, kind: ProductionKind, input: &str) -> ParserOutput {{
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
               use strum_macros::*;
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

                        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                        pub enum TokenKind {{
                            {token_kinds}
                        }}

                        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                        pub enum RuleKind {{
                            {rule_kinds}
                        }}

                        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display)]
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
