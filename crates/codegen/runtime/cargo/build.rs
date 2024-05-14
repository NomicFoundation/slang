use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;

fn main() -> Result<()> {
    OutputLanguage::Cargo.generate_stubs()
}
