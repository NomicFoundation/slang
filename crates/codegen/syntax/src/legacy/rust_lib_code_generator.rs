use std::path::Path;

use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use infra_utils::{cargo::CargoWorkspace, codegen::Codegen};
use quote::quote;

use crate::legacy::code_generator::CodeGenerator;

impl CodeGenerator {
    pub fn write_rust_lib_sources(
        &self,
        language: &LanguageDefinition,
        output_dir: &Path,
    ) -> Result<()> {
        let templates_dir =
            CargoWorkspace::locate_source_crate("codegen_legacy_syntax_templates")?.join("src");

        let mut codegen = Codegen::read_write(&templates_dir)?;

        self.write_common_sources(&mut codegen, &templates_dir, output_dir)?;

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
               pub mod language;
               pub mod parse_error;
               pub mod parse_output;
               pub mod text_index;
               pub mod visitor;
            };

            codegen.write_file(output_dir.join("mod.rs"), content.to_string())?;
        }

        codegen.copy_file(
            templates_dir.join("rust/parse_output.rs"),
            output_dir.join("parse_output.rs"),
        )?;

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

                #[derive(Debug, Eq, PartialEq, thiserror::Error)]
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
                language_boilerplate_common = codegen.read_file(templates_dir.join("shared/language.rs"))?,
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

            codegen.write_file(output_dir.join("language.rs"), content)?;
        }

        return Ok(());
    }
}
