mod kinds;

use std::path::Path;

use anyhow::Result;
use codegen_schema::types::LanguageDefinitionRef;
use infra_utils::{cargo::CargoWorkspace, codegen::Codegen};

use crate::nodes::kinds::NodeKindsTemplate;

pub fn generate_syntax_nodes_mod(
    language: &LanguageDefinitionRef,
    output_dir: &Path,
) -> Result<()> {
    let templates_dir = CargoWorkspace::locate_source_crate("codegen_syntax")?.join("src/nodes");
    let mut codegen = Codegen::read_write(&templates_dir)?;

    let template_path = templates_dir.join("kinds/template.tera");

    codegen.render(
        NodeKindsTemplate::collect_production_kinds(language),
        &template_path,
        output_dir.join("production_kind.rs"),
    )?;

    codegen.render(
        NodeKindsTemplate::collect_rule_kinds(language),
        &template_path,
        output_dir.join("rule_kind.rs"),
    )?;

    codegen.render(
        NodeKindsTemplate::collect_token_kinds(language),
        &template_path,
        output_dir.join("token_kind.rs"),
    )?;

    codegen.write_file(
        output_dir.join("mod.rs"),
        "
          pub mod production_kind;
          pub mod rule_kind;
          pub mod token_kind;
        ",
    )?;

    return Ok(());
}
