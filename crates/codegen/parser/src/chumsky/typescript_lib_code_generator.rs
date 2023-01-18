use std::path::PathBuf;

use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::{code_generator::CodeGenerator, typescript_lib_boilerplate};

impl CodeGenerator {
    pub fn write_typescript_lib_sources(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        codegen
            .write_file(
                &output_dir.join("mod.rs"),
                &typescript_lib_boilerplate::mod_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("lex.rs"),
                &typescript_lib_boilerplate::lex_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("cst.rs"),
                &typescript_lib_boilerplate::cst_head().to_string(),
            )
            .unwrap();

        self.write_parser_source(grammar, codegen, output_dir);

        codegen
            .write_file(
                &output_dir.join("language.rs"),
                &typescript_lib_boilerplate::language_head().to_string(),
            )
            .unwrap();

        // Do the kinds last, because the code generation steps above may have added new kinds
        codegen
            .write_file(
                &output_dir.join("kinds.rs"),
                &format!(
                    "{}

                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #[napi]
                    pub enum TokenKind {{
                        {}
                    }}

                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #[napi]
                    pub enum RuleKind {{
                        {}
                    }}
                    
                    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                    #[napi]
                    pub enum ProductionKind {{
                        {}
                    }}
                    ",
                    typescript_lib_boilerplate::kinds_head(),
                    self.token_kinds
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                    self.rule_kinds
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                    self.parsers
                        .iter()
                        .map(|(name, _)| name.clone())
                        .collect::<Vec<_>>()
                        .join(",\n"),
                ),
            )
            .unwrap();
    }
}
