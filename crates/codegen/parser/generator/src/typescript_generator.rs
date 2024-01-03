use std::path::Path;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;
use serde::Serialize;

use crate::ast_model::AstModel;

pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    pub fn generate(ast_model: &AstModel, output_dir: &Path) -> Result<()> {
        let runtime_dir =
            CargoWorkspace::locate_source_crate("codegen_parser_runtime")?.join("src");

        let mut codegen = Codegen::read_write(&runtime_dir)?;

        {
            #[derive(Serialize)]
            pub struct Template<'a> {
                pub ast_model: &'a AstModel,
            }
            codegen.render(
                Template { ast_model },
                runtime_dir.join("napi/templates/ast_types.ts.jinja2"),
                output_dir.join("src/ast/generated/ast_types.ts"),
            )?;
        }

        Ok(())
    }
}
