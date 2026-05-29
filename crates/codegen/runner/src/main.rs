use anyhow::Result;
use codegen_generator::{RuntimeModel, RuntimeModelV2};
use codegen_spec::Spec;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use language_definition::model::Language;
use language_v2_definition::model::Language as LanguageV2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;
use solidity_v2_language::SolidityDefinition as SolidityDefinitionV2;

fn main() {
    [
        || generate_solidity_v1_spec(),
        || generate_solidity_v1_builtins(),
        || {
            generate_tera_templates_v1(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "slang_solidity",
            )
        },
        || {
            generate_tera_templates_v1(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "solidity_cargo_wasm",
            )
        },
        || {
            generate_tera_templates_v1(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "solidity_npm_package",
            )
        },
        || {
            generate_tera_templates_v1(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "solidity_cargo_tests",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_ast",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_cst",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_parser",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_common",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_ir",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_semantic",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "solidity_v2_testing_utils",
            )
        },
        || {
            generate_tera_templates_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "solidity_v2_cargo_tests",
            )
        },
    ]
    .par_iter()
    .for_each(|op| op().unwrap());
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

fn generate_tera_templates_v1(
    fs: &mut CodegenFileSystem,
    language: &Language,
    crate_name: &str,
) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;
    let model = RuntimeModel::from_language(language)?;

    CodegenRuntime::render_templates_in_place(fs, &crate_dir, model)
}

fn generate_tera_templates_v2(
    fs: &mut CodegenFileSystem,
    language: &LanguageV2,
    crate_name: &str,
) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;
    let model = RuntimeModelV2::from_language(language);

    CodegenRuntime::render_templates_in_place(fs, &crate_dir, model)
}
