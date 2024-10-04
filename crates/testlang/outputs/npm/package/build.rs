use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;
use infra_utils::cargo::CargoWorkspace;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    let output_dir =
        CargoWorkspace::locate_source_crate("testlang_npm_package")?.join("src/generated");

    OutputLanguage::Npm.generate_runtime(&language, &output_dir, |_| Ok(()))
}
