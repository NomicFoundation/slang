mod rules;
mod visitors;

use codegen_utils::errors::CodegenResult;

use crate::types::SchemaRef;

pub fn validate_schema(schema: &SchemaRef) -> CodegenResult<()> {
    // Validation should stop at each step if there are any errors:

    rules::language_versions::run(schema)?;
    rules::definitions::run(schema)?;
    rules::references::run(schema)?;
    rules::empty_roots::run(schema)?;

    return Ok(());
}
