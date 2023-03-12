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

        {
            let content = quote! {
               pub mod kinds;
               pub mod cst;
               pub mod cst_visitor;
               pub mod language;
               pub mod parser_output;
               mod scanners;
               mod parsers;
            };

            codegen
                .write_file(&output_dir.join("mod.rs"), &content.to_string())
                .unwrap();
        }

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
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                {language_boilerplate_common}

                pub struct Language {{
                    version: Version,
                    {version_flag_declarations}
                }}

                #[derive(thiserror::Error, Debug)]
                pub enum Error {{
                    #[error(\"Invalid {grammar_title} language version '{{0}}'.\")]
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
                ",
                language_boilerplate_common = codegen
                    .read_file(
                        &codegen
                            .repo_root
                            .join("crates/codegen/parser_templates/src/shared/language.rs"),
                    )
                    .unwrap(),
                version_flag_declarations = self.version_flag_declarations(),
                grammar_title = &grammar.title,
                versions_array = {
                    let versions = grammar.versions.iter().map(|v| v.to_string());
                    quote! { static VERSIONS: &'static [&'static str] = &[ #(#versions),* ]; }
                },
                version_flag_initializers = self.version_flag_initializers(),
                trampolines = quote! {
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
                },
                scanner_invocations = self.scanner_invocations(),
                parser_invocations = self.parser_invocations(),
            );

            codegen
                .write_file(&output_dir.join("language.rs"), &content)
                .unwrap();
        }

        // Do the kinds last, because the code generation steps above may have added new kinds
        {
            let content = {
                let token_kinds = self.token_kinds();
                let rule_kinds = self.rule_kinds();
                let production_kinds = self.production_kinds();
                quote! {
                    use serde::Serialize;
                    use strum_macros::*;

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                    #token_kinds

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                    #rule_kinds

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display)]
                    #production_kinds
                }
            };

            codegen
                .write_file(&output_dir.join("kinds.rs"), &content.to_string())
                .unwrap();
        }
    }
}
