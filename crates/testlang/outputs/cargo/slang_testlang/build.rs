use anyhow::Result;
use codegen_parser_generator::RustGenerator;
use infra_utils::cargo::CargoWorkspace;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    RustGenerator::generate(
        &language,
        &CargoWorkspace::locate_source_crate("slang_testlang")?.join("src/generated"),
    )
}
