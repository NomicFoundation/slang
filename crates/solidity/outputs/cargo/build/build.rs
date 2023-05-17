use anyhow::Result;
use codegen_schema::types::schema::Schema;
use codegen_syntax::SyntaxGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SoliditySchemaExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let schema = Schema::load_solidity()?;

        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/cargo/crate/src/generated");

        schema.generate_rust_lib_sources(codegen, &output_dir);
        return Ok(());
    });
}
