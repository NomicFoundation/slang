use std::path::PathBuf;

use codegen_schema::types::LanguageDefinition;
use codegen_utils::context::CodegenContext;
use quote::quote;

use crate::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_typescript_lib_sources(
        &self,
        language: &LanguageDefinition,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        self.write_common_sources(codegen, output_dir);

        {
            let content = quote! {
               pub mod kinds;
               pub mod cst;
               pub mod cst_types;
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
                    .join("crates/codegen/syntax_templates/src/typescript/cst_types.rs"),
                &output_dir.join("cst_types.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/syntax_templates/src/typescript/parser_output.rs"),
                &output_dir.join("parser_output.rs"),
            )
            .unwrap();

        {
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                {language_boilerplate_common}

                #[napi]
                pub struct Language {{
                    pub(crate) version: Version,
                    {version_flag_declarations}
                }}

                #[derive(thiserror::Error, Debug)]
                pub enum Error {{
                    // Shared with Rust
                    #[error(\"Unsupported {language_title} language version '{{0}}'.\")]
                    UnsupportedLanguageVersion(Version),
                    #[error(\"Production '{{0:?}}' is not valid in this version of {language_title}.\")]
                    InvalidProductionVersion(ProductionKind),

                    // TypeScript-specific
                    #[error(\"Invalid semantic version '{{0}}'.\")]
                    InvalidSemanticVersion(String),
                }}

                impl From<Error> for napi::Error {{
                    fn from(value: Error) -> Self {{
                        napi::Error::from_reason(value.to_string())
                    }}
                }}

                {versions_array}

                #[napi]
                impl Language {{
                    #[napi(constructor)]
                    pub fn new(version: String) -> Result<Self, napi::Error> {{
                        let version = Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
                        if VERSIONS.contains(&version.to_string().as_str()) {{
                            Ok(Self {{
                                {version_flag_initializers},
                                version,
                            }})
                        }} else {{
                            Err(Error::UnsupportedLanguageVersion(version).into())
                        }}
                    }}

                    #[napi(getter)]
                    pub fn version(&self) -> String {{
                        self.version.to_string()
                    }}

                    #[napi]
                    pub fn supported_versions() -> Vec<String> {{
                        return VERSIONS.iter().map(|v| v.to_string()).collect();
                    }}

                    #[napi]
                    pub fn parse(&self, production_kind: ProductionKind, input: String) -> Result<ParseOutput, napi::Error> {{
                        let input = input.as_str();
                        match production_kind {{
                            {scanner_invocations},
                            {parser_invocations},
                        }}.ok_or_else(|| Error::InvalidProductionVersion(production_kind).into())
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
                versions_array = {
                    let versions = language.versions.iter().map(|v| v.to_string());
                    quote! { const VERSIONS: &'static [&'static str] = &[ #(#versions),* ]; }
                },
                version_flag_initializers = self.version_flag_initializers(),
                language_title = &language.title,
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
                    use napi::bindgen_prelude::*;
                    use napi_derive::napi;

                    #[napi]
                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #token_kinds

                    #[napi]
                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #rule_kinds

                    #[napi]
                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #production_kinds
                }
            };

            codegen
                .write_file(&output_dir.join("kinds.rs"), &content.to_string())
                .unwrap();
        }
    }
}
