use std::path::PathBuf;

use codegen_schema::types::LanguageDefinition;
use codegen_utils::context::CodegenContext;
use quote::quote;

use crate::legacy::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_napi_lib_sources(
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
               pub mod cst_ts_wrappers;
               pub mod cursor;
               pub mod cursor_ts_wrappers;
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
                    .join("crates/codegen/legacy_syntax_templates/src/napi/cst_ts_wrappers.rs"),
                &output_dir.join("cst_ts_wrappers.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/napi/cursor_ts_wrappers.rs"),
                &output_dir.join("cursor_ts_wrappers.rs"),
            )
            .unwrap();

        codegen
            .copy_file(
                &codegen
                    .repo_root
                    .join("crates/codegen/legacy_syntax_templates/src/napi/parse_output.rs"),
                &output_dir.join("parse_output.rs"),
            )
            .unwrap();

        {
            // Use `format!` here because the content contains comments, that `quote!` throws away.
            let content = format!(
                "
                {language_boilerplate_common}

                #[napi(namespace = \"legacy\")]
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

                #[napi(namespace = \"legacy\")]
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
                            .join("crates/codegen/legacy_syntax_templates/src/shared/language.rs"),
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
    }
}
