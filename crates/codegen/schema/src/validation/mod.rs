mod rules;
mod visitors;

use codegen_utils::errors::CodegenResult;

use crate::types::LanguageDefinitionRef;

pub fn validate_language(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    // Validation should stop at each step if there are any errors:

    rules::language_versions::run(language)?;
    rules::definitions::run(language)?;
    rules::references::run(language)?;
    rules::lints::run(language)?;

    return Ok(());
}
