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
                    pub(crate) version: Version,
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

                    pub fn parse(&self, production_kind: ProductionKind, input: &str) -> ParserOutput {{
                        let output = match production_kind {{
                            {scanner_invocations},
                            {parser_invocations},
                        }};
                        
                        output.unwrap_or_else(|| {{
                            let message = format!(\"ProductionKind {{production_kind}} is not valid in this version of {grammar_title}\");
                            ParserOutput {{
                                parse_tree: None,
                                errors: vec![ParseError::new(0, message)]
                            }}
                        }})
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
