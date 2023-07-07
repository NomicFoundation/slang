mod keywords;
mod parsers;
mod productions;
mod versions;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::definitions::{
            keywords::Keywords, parsers::Parsers, productions::Productions, versions::Versions,
        },
        visitors::Reporter,
    },
};
use codegen_utils::errors::CodegenResult;

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    Productions::validate(language, &mut reporter);
    Versions::validate(language, &mut reporter);
    Parsers::validate(language, &mut reporter);
    Keywords::validate(language, &mut reporter);

    return reporter.to_result();
}
