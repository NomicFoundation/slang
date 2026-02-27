use anyhow::Result;
use codegen_generator::RuntimeGenerator;
use codegen_spec::Spec;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use language_definition::model::Language;
use language_v2_definition::model::Language as LanguageV2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solidity_language::SolidityDefinition;
use solidity_v2_language::SolidityDefinition as SolidityDefinitionV2;

fn main() {
    // TODO(#1286):
    // Using a cargo binary rather than a build task, to avoid invalidating the build tree when only the inputs change.
    // This is a poor man's version of a "build script" that runs all codegen steps in parallel.
    // It is temporary step, and should be simplified/removed in subsequent PRs.

    [
        || generate_solidity_spec(),
        || generate_solidity_tests(),
        || generate_solidity_v2_tests(),
        || {
            let mut fs = CodegenFileSystem::default();
            let language = SolidityDefinition::create();

            generate_in_place(&mut fs, &language, "slang_solidity")?;

            generate_builtins(&mut fs, &language, "slang_solidity")?;

            Ok(())
        },
        || {
            generate_in_place(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "solidity_cargo_wasm",
            )
        },
        || {
            generate_in_place(
                &mut CodegenFileSystem::default(),
                &SolidityDefinition::create(),
                "solidity_npm_package",
            )
        },
        || {
            generate_in_place_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_cst",
            )
        },
        || {
            generate_in_place_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_parser",
            )
        },
        || {
            generate_in_place_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "slang_solidity_v2_common",
            )
        },
        || {
            generate_in_place_v2(
                &mut CodegenFileSystem::default(),
                &SolidityDefinitionV2::create(),
                "solidity_v2_testing_utils",
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

    lang_def.generate_binder_tests(
        &snapshots_crate.join("bindings_output"),
        &tests_crate.join("src/binder/generated"),
    )?;

    Ok(())
}

fn generate_solidity_v2_tests() -> Result<()> {
    let v1_snapshots = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?;
    let v2_snapshots = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?;
    let tests_crate = CargoWorkspace::locate_source_crate("solidity_v2_cargo_tests")?;

    // Step 1: Migrate v1 tests to v2 format
    solidity_v2_testing_utils::migration::migrate_v1_tests_to_v2(
        &v1_snapshots.join("cst_output"),
        &v2_snapshots.join("cst_output/generated/from_v1/SourceUnit"),
    )?;

    // Step 2: Generate test harness from migrated snapshots
    codegen_testing::cst_output::generate_cst_output_tests_for_generated_inputs(
        &v2_snapshots.join("cst_output/generated/from_v1/SourceUnit"),
        &tests_crate.join("src/cst/cst_output/generated"),
    )?;

    Ok(())
}

fn generate_in_place(
    fs: &mut CodegenFileSystem,
    language: &Language,
    crate_name: &str,
) -> Result<()> {
    let the_crate = CargoWorkspace::locate_source_crate(crate_name)?;

    RuntimeGenerator::generate_templates_in_place(language, fs, &the_crate)
}

fn generate_in_place_v2(
    fs: &mut CodegenFileSystem,
    language: &LanguageV2,
    crate_name: &str,
) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;

    RuntimeGenerator::generate_templates_in_place_v2(language, fs, &crate_dir)
}

fn generate_builtins(
    fs: &mut CodegenFileSystem,
    language: &Language,
    output_crate: &str,
) -> Result<()> {
    let file_path = CargoWorkspace::locate_source_crate(output_crate)?
        .join("src/bindings/built_ins.generated.rs");

    let contents = solidity_language::render_built_ins(language)?;

    fs.write_file_formatted(file_path, contents)
}
