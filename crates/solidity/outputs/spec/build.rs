use anyhow::Result;
use codegen_schema::types::Schema;
use codegen_spec::SpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SoliditySchemaExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let schema = Schema::load_solidity()?;
        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/spec/generated");

        return schema.generate_spec(codegen, &output_dir);
    });
}
