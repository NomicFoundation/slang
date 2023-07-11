mod kinds;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;

use crate::nodes::kinds::{generate_production_kind, generate_rule_kind, generate_token_kind};

pub fn generate_syntax_nodes_mod(
    language: &LanguageDefinitionRef,
    codegen: &mut CodegenContext,
    syntax_nodes_dir: &PathBuf,
) -> Result<()> {
    codegen.write_file(
        &syntax_nodes_dir.join("production_kind.rs"),
        &generate_production_kind(language),
    )?;

    codegen.write_file(
        &syntax_nodes_dir.join("rule_kind.rs"),
        &generate_rule_kind(language),
    )?;

    codegen.write_file(
        &syntax_nodes_dir.join("token_kind.rs"),
        &generate_token_kind(language),
    )?;

    codegen.write_file(
        &syntax_nodes_dir.join("mod.rs"),
        "
          pub mod production_kind;
          pub mod rule_kind;
          pub mod token_kind;
        ",
    )?;

    return Ok(());
}
