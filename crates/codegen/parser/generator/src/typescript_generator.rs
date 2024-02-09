use std::path::Path;

use anyhow::Result;
use codegen_language_definition::model::Language;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;
use serde::Serialize;

use crate::ast_model::AstModel;

pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    pub fn generate(language: &Language, output_dir: &Path) -> Result<()> {
        let runtime_dir =
            CargoWorkspace::locate_source_crate("codegen_parser_runtime")?.join("src");

        let mut codegen = Codegen::read_write(&runtime_dir)?;

        {
            #[derive(Serialize)]
            struct Context {
                ast_model: AstModel,
            }
            codegen.render(
                Context {
                    ast_model: AstModel::create(language),
                },
                runtime_dir.join("napi_interface/templates/ast_types.ts.jinja2"),
                output_dir.join("src/ast/generated/ast_types.ts"),
            )?;
        }

        Ok(())
    }
}
