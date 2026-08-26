use anyhow::Result;
use codegen_v2_generator::RuntimeModel;
use infra_utils::codegen::CodegenRuntime;
use solidity_v2_language::SolidityDefinition;

fn main() -> Result<()> {
    generate_tera_templates()
}

fn generate_tera_templates() -> Result<()> {
    let language = SolidityDefinition::create();
    let model = RuntimeModel::from_language(&language);

    let mut context = tera::Context::new();
    context.insert("model", &model);

    CodegenRuntime::V2Templates.render_templates(&context)
}
