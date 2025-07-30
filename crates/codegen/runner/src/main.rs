mod passes;

use anyhow::Result;
use codegen_language_definition::model::Language;
use codegen_runtime_generator::RuntimeGenerator;
use codegen_spec::Spec;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;

use crate::passes::generate_passes;

fn main() {
    // TODO(#1286):
    // Using a cargo binary rather than a build task, to avoid invalidating the build tree when only the inputs change.
    // This is a poor man's version of a "build script" that runs all codegen steps in parallel.
    // It is temporary step, and should be simplified/removed in subsequent PRs.

    [
        || generate_solidity_spec(),
        || generate_solidity_tests(),
        || generate_stubs("codegen_runtime_cargo_crate"),
        || generate_stubs("codegen_runtime_cargo_wasm"),
        || generate_stubs("codegen_runtime_npm_package"),
        || {
            let mut fs = CodegenFileSystem::default();
            let language = SolidityDefinition::create();

            generate_product(
                &mut fs,
                &language,
                "codegen_runtime_cargo_crate",
                "slang_solidity",
            )?;

            generate_builtins(&mut fs, &language, "slang_solidity")?;

            generate_passes(
                &mut fs,
                &language,
                "codegen_runtime_cargo_crate",
                "slang_solidity",
            )?;

            Ok(())
        },
        || {
            generate_product(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "codegen_runtime_cargo_wasm",
                "solidity_cargo_wasm",
            )
        },
        || {
            generate_product(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "codegen_runtime_npm_package",
                "solidity_npm_package",
            )
        },
    ]
    .par_iter()
    .for_each(|op| op().unwrap());
}

fn generate_solidity_spec() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    Spec::generate(language, &output_dir)
}

fn generate_solidity_tests() -> Result<()> {
    let lang_def = SolidityDefinition::create();
    let snapshots_crate = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?;
    let tests_crate = CargoWorkspace::locate_source_crate("solidity_cargo_tests")?;

    lang_def.generate_version_breaks(&tests_crate.join("src/cst/generated"))?;

    lang_def.generate_bindings_output_tests(
        &snapshots_crate.join("bindings_output"),
        &tests_crate.join("src/bindings/bindings_output/generated"),
    )?;

    lang_def.generate_cst_output_tests(
        &snapshots_crate.join("cst_output"),
        &tests_crate.join("src/cst/cst_output/generated"),
    )?;

    Ok(())
}

fn generate_stubs(crate_name: &str) -> Result<()> {
    let mut fs = CodegenFileSystem::default();
    let source_dir = CargoWorkspace::locate_source_crate(crate_name)?;

    RuntimeGenerator::generate_stubs(&mut fs, &source_dir)
}

fn generate_product(
    fs: &mut CodegenFileSystem,
    language: &Language,
    input_crate: &str,
    output_crate: &str,
) -> Result<()> {
    let input_dir = CargoWorkspace::locate_source_crate(input_crate)?.join("src/runtime");
    let output_dir = CargoWorkspace::locate_source_crate(output_crate)?.join("src/generated");

    RuntimeGenerator::generate_product(language, fs, &input_dir, &output_dir)
}

fn generate_builtins(
    fs: &mut CodegenFileSystem,
    language: &Language,
    output_crate: &str,
) -> Result<()> {
    let file_path = CargoWorkspace::locate_source_crate(output_crate)?
        .join("src/generated/extensions/built_ins.rs");

    let contents = solidity_language::render_built_ins(language)?;

    fs.write_file_formatted(file_path, contents)
}
