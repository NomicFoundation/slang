use std::path::PathBuf;

use codegen_schema::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::{boilerplate, code_generator::CodeGenerator, rust_lib_boilerplate};

impl CodeGenerator {
    pub fn write_rust_lib_sources(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        codegen
            .write_file(
                &output_dir.join("mod.rs"),
                &rust_lib_boilerplate::mod_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("lex.rs"),
                &boilerplate::lex_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("cst.rs"),
                &format!(
                    "{}\n{}",
                    boilerplate::cst_head(),
                    rust_lib_boilerplate::cst_visitor_head()
                ),
            )
            .unwrap();

        self.write_parser_source(grammar, codegen, output_dir);

        codegen
            .write_file(
                &output_dir.join("language.rs"),
                &rust_lib_boilerplate::language_head().to_string(),
            )
            .unwrap();

        // Do the kinds last, because the code generation steps above may have added new kinds
        codegen
            .write_file(
                &output_dir.join("kinds.rs"),
                &format!(
                    "{}

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                    pub enum TokenKind {{
                        {}
                    }}

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
                    pub enum RuleKind {{
                        {}
                    }}

                    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display)]
                    pub enum ProductionKind {{
                        {}
                    }}
                    ",
                    rust_lib_boilerplate::kinds_head(),
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
