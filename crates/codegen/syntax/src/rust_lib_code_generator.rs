use std::path::PathBuf;

use codegen_schema::types::LanguageDefinition;
use codegen_utils::context::CodegenContext;
use quote::quote;

use crate::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_rust_lib_sources(
        &self,
        language: &LanguageDefinition,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        self.write_common_sources(codegen, output_dir);

        {
            let content = quote! {
               #[macro_use]
               mod scanner_macros;

               mod parser_function;
               mod parser_helpers;
               mod parser_result;
               mod parsers;
               mod scanner_function;
               mod scanners;
               mod stream;

               pub mod cst;
               pub mod cursor;
               pub mod kinds;
               pub mod language;
               pub mod parse_error;
               pub mod parse_output;
               pub mod text_index;
               pub mod visitor;
            };

            codegen
                .write_file(&output_dir.join("mod.rs"), &content.to_string())
                .unwrap();
        }

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/syntax_templates/src/rust/parse_output.rs"),
                &output_dir.join("parse_output.rs"),
            )
            .unwrap();

        {
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                {language_boilerplate_common}

                #[derive(Debug)]
                pub struct Language {{
                    pub(crate) version: Version,
                    {version_flag_declarations}
                }}

                #[derive(thiserror::Error, Debug)]
                pub enum Error {{
                    #[error(\"Unsupported {language_title} language version '{{0}}'.\")]
                    UnsupportedLanguageVersion(Version),
                    #[error(\"Production '{{0:?}}' is not valid in this version of {language_title}.\")]
                    InvalidProductionVersion(ProductionKind),
                }}

                {versions_array}

                impl Language {{
                    pub fn new(version: Version) -> Result<Self, Error> {{
                        if VERSIONS.contains(&version.to_string().as_str()) {{
                            Ok(Self {{
                                {version_flag_initializers},
                                version,
                            }})
                        }} else {{
                            Err(Error::UnsupportedLanguageVersion(version))
                        }}
                    }}

                    pub fn version(&self) -> &Version {{
                        &self.version
                    }}

                    pub fn supported_versions() -> Vec<Version> {{
                        return VERSIONS.iter().map(|v| Version::parse(v).unwrap()).collect();
                    }}

                    pub fn parse(&self, production_kind: ProductionKind, input: &str) -> Result<ParseOutput, Error> {{
                        let output = match production_kind {{
                            {scanner_invocations},
                            {parser_invocations},
                        }};
                        
                        output.ok_or_else(|| Error::InvalidProductionVersion(production_kind))
                    }}
                }}
                ",
                language_boilerplate_common = codegen
                    .read_file(
                        &codegen
                            .repo_root
                            .join("crates/codegen/syntax_templates/src/shared/language.rs"),
                    )
                    .unwrap(),
                version_flag_declarations = self.version_flag_declarations(),
                language_title = &language.title,
                versions_array = {
                    let versions = language.versions.iter().map(|v| v.to_string());
                    quote! { const VERSIONS: &'static [&'static str] = &[ #(#versions),* ]; }
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

                    #[derive(
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        PartialOrd,
                        Ord,
                        Serialize,
                        strum_macros::EnumString,
                        strum_macros::AsRefStr,
                        strum_macros::Display,
                    )]
                    #token_kinds

                    #[derive(
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        PartialOrd,
                        Ord,
                        Serialize,
                        strum_macros::EnumString,
                        strum_macros::AsRefStr,
                        strum_macros::Display,
                    )]
                    #rule_kinds

                    #[derive(
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        PartialOrd,
                        Ord,
                        Serialize,
                        strum_macros::EnumString,
                        strum_macros::AsRefStr,
                        strum_macros::Display,
                    )]
                    #production_kinds
                }
            };

            codegen
                .write_file(&output_dir.join("kinds.rs"), &content.to_string())
                .unwrap();
        }
    }
}
