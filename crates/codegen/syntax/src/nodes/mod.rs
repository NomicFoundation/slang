mod kinds;

use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use codegen_utils::context::CodegenContext;
use tera::Tera;

use crate::{nodes::kinds::NodeKindsTemplate, templates::TemplateContext};

pub fn generate_syntax_nodes_mod(
    language: &LanguageDefinitionRef,
    tera: &Tera,
    codegen: &mut CodegenContext,
    syntax_nodes_dir: &PathBuf,
) -> Result<()> {
    codegen.write_file(
        &syntax_nodes_dir.join("production_kind.rs"),
        &NodeKindsTemplate::collect_production_kinds(language).render(tera)?,
    )?;

    codegen.write_file(
        &syntax_nodes_dir.join("rule_kind.rs"),
        &NodeKindsTemplate::collect_rule_kinds(language).render(tera)?,
    )?;

    codegen.write_file(
        &syntax_nodes_dir.join("token_kind.rs"),
        &NodeKindsTemplate::collect_token_kinds(language).render(tera)?,
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
