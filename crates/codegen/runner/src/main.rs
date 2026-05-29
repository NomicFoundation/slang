use std::path::Path;

use anyhow::Result;
use codegen_generator::{RuntimeModel, RuntimeModelV2};
use codegen_spec::Spec;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use infra_utils::paths::PathExtensions;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;
use solidity_v2_language::SolidityDefinition as SolidityDefinitionV2;

fn main() {
    [
        || generate_solidity_spec_v1(),
        || generate_solidity_builtins_v1(),
        || generate_tera_templates_v1(),
        || generate_tera_templates_v2(),
    ]
    .par_iter()
    .for_each(|op| op().unwrap());
}

fn generate_solidity_spec_v1() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    Spec::generate(language, &output_dir)
}

fn generate_solidity_builtins_v1() -> Result<()> {
    let language = SolidityDefinition::create();
    let contents = solidity_language::render_built_ins(&language)?;

    let file_path = CargoWorkspace::locate_source_crate("slang_solidity")?
        .join("src/bindings/built_ins.generated.rs");

    let mut fs = CodegenFileSystem::default();
    fs.write_file_formatted(file_path, contents)
}

fn generate_tera_templates_v1() -> Result<()> {
    let language = SolidityDefinition::create();
    let model = RuntimeModel::from_language(&language)?;

    CodegenRuntime::render_templates_in_place(model, |template_path| {
        classify_template(template_path) == TemplateVersion::V1
    })
}

fn generate_tera_templates_v2() -> Result<()> {
    let language = SolidityDefinitionV2::create();
    let model = RuntimeModelV2::from_language(&language);

    CodegenRuntime::render_templates_in_place(model, |template_path| {
        classify_template(template_path) == TemplateVersion::V2
    })
}

#[derive(PartialEq, Eq)]
enum TemplateVersion {
    V1,
    V2,
}

fn classify_template(template_path: &Path) -> TemplateVersion {
    match template_path.strip_repo_root() {
        Ok(p) if p.starts_with("crates/language/") || p.starts_with("crates/solidity/") => {
            TemplateVersion::V1
        }
        Ok(p) if p.starts_with("crates/language-v2/") || p.starts_with("crates/solidity-v2/") => {
            TemplateVersion::V2
        }
        _ => {
            panic!("Cannot categorize template (Slang v1 or v2?): {template_path:?}");
        }
    }
}
