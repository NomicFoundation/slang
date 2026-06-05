use anyhow::Result;
use codegen_generator::{RuntimeModel, RuntimeModelV2};
use codegen_spec::Spec;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use infra_utils::paths::PathExtensions;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;
use solidity_v2_language::SolidityDefinition as SolidityDefinitionV2;

fn main() -> Result<()> {
    [
        || generate_solidity_v1_spec(),
        || generate_solidity_v1_builtins(),
        || generate_tera_templates(),
    ]
    .par_iter()
    .try_for_each(|op| op())
}

fn generate_solidity_v1_spec() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    Spec::generate(language, &output_dir)
}

fn generate_solidity_v1_builtins() -> Result<()> {
    let language = SolidityDefinition::create();
    let contents = solidity_language::render_built_ins(&language)?;

    let file_path = CargoWorkspace::locate_source_crate("slang_solidity")?
        .join("src/bindings/built_ins.generated.rs");

    let mut fs = CodegenFileSystem::default();
    fs.write_file_formatted(file_path, contents)
}

fn generate_tera_templates() -> Result<()> {
    let v1_context = {
        let language = SolidityDefinition::create();
        let model = RuntimeModel::from_language(&language)?;

        let mut context = tera::Context::new();
        context.insert("model", &model);
        context
    };

    let v2_context = {
        let language = SolidityDefinitionV2::create();
        let model = RuntimeModelV2::from_language(&language);

        let mut context = tera::Context::new();
        context.insert("model", &model);
        context
    };

    CodegenRuntime::render_templates_in_place(|template_path| {
        match template_path.strip_repo_root() {
            Ok(p) if p.starts_with("crates/language/") => &v1_context,
            Ok(p) if p.starts_with("crates/solidity/") => &v1_context,
            Ok(p) if p.starts_with("crates/language-v2/") => &v2_context,
            Ok(p) if p.starts_with("crates/solidity-v2/") => &v2_context,

            _ => panic!("Cannot categorize template: {template_path:?}"),
        }
    })
}
