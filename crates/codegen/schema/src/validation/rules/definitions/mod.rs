mod keywords;
mod operators;
mod productions;
mod versions;

use anyhow::Result;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::definitions::{
            keywords::Keywords, operators::Operators, productions::Productions, versions::Versions,
        },
        visitors::Reporter,
    },
};

pub fn run(language: &LanguageDefinitionRef) -> Result<()> {
    let mut reporter = Reporter::new();

    Keywords::validate(language, &mut reporter);
    Operators::validate(language, &mut reporter);
    Productions::validate(language, &mut reporter);
    Versions::validate(language, &mut reporter);

    reporter.into_result()
}
