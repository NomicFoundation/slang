use anyhow::Result;
use codegen_parser_generator::TypeScriptGenerator;
use infra_utils::cargo::CargoWorkspace;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    TypeScriptGenerator::generate(
        &language,
        &CargoWorkspace::locate_source_crate("testlang_npm_package")?,
    )
}
