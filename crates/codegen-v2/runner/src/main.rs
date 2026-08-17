use anyhow::Result;
use codegen_v2_generator::RuntimeModel;
use infra_utils::codegen::CodegenRuntime;
use infra_utils::paths::PathExtensions;
use solidity_v2_language::SolidityDefinition;

fn main() -> Result<()> {
    generate_tera_templates()
}

fn generate_tera_templates() -> Result<()> {
    let language = SolidityDefinition::create();
    let model = RuntimeModel::from_language(&language);

    let mut context = tera::Context::new();
    context.insert("model", &model);

    CodegenRuntime::render_templates_in_place(|template_path| {
        match template_path.strip_repo_root().unwrap() {
            // Ignore V1 templates:
            p if p.starts_with("crates/language/") => None,
            p if p.starts_with("crates/solidity/") => None,

            // Process V2 templates:
            p if p.starts_with("crates/language-v2/") => Some(&context),
            p if p.starts_with("crates/solidity-v2/") => Some(&context),

            _ => panic!("Cannot categorize template: {template_path:?}"),
        }
    })
}
