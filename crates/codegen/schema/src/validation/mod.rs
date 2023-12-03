mod rules;
mod visitors;

use anyhow::Result;

use crate::types::LanguageDefinitionRef;

pub fn validate_language(language: &LanguageDefinitionRef) -> Result<()> {
    // Validation should stop at each step if there are any errors:

    rules::definitions::run(language)?;
    rules::references::run(language)?;
    rules::lints::run(language)?;

    Ok(())
}
