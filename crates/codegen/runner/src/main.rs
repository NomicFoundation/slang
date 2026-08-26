use anyhow::Result;
use codegen_generator::RuntimeModel;
use codegen_spec::Spec;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    [
        || generate_solidity_spec(),
        || generate_solidity_builtins(),
        || generate_tera_templates(),
    ]
    .par_iter()
    .try_for_each(|op| op())
}

fn generate_solidity_spec() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    Spec::generate(language, &output_dir)
}

fn generate_solidity_builtins() -> Result<()> {
    let language = SolidityDefinition::create();
    let contents = solidity_language::render_built_ins(&language)?;

    let file_path = CargoWorkspace::locate_source_crate("slang_solidity")?
        .join("src/bindings/built_ins.generated.rs");

    let mut fs = CodegenFileSystem::default();
    fs.write_file_formatted(file_path, contents)
}

fn generate_tera_templates() -> Result<()> {
    let language = SolidityDefinition::create();
    let model = RuntimeModel::from_language(&language)?;

    let mut context = tera::Context::new();
    context.insert("model", &model);

    CodegenRuntime::V1Templates.render_templates(&context)
}
