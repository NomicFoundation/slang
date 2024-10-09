use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;

fn main() -> Result<()> {
    let source_dir = CargoWorkspace::locate_source_crate("codegen_runtime_npm_package")?;

    RuntimeGenerator::generate_stubs(&source_dir)
}
