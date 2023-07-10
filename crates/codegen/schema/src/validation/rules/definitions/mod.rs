mod keywords;
mod operators;
mod productions;
mod versions;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::definitions::{
            keywords::Keywords, operators::Operators, productions::Productions, versions::Versions,
        },
        visitors::Reporter,
    },
};
use codegen_utils::errors::CodegenResult;

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    Keywords::validate(language, &mut reporter);
    Operators::validate(language, &mut reporter);
    Productions::validate(language, &mut reporter);
    Versions::validate(language, &mut reporter);

    return reporter.to_result();
}
