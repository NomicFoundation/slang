mod keywords;
mod operators;
mod productions;
mod versions;

use anyhow::Result;

use crate::types::LanguageDefinitionRef;
use crate::validation::rules::definitions::keywords::Keywords;
use crate::validation::rules::definitions::operators::Operators;
use crate::validation::rules::definitions::productions::Productions;
use crate::validation::rules::definitions::versions::Versions;
use crate::validation::visitors::Reporter;

pub fn run(language: &LanguageDefinitionRef) -> Result<()> {
    let mut reporter = Reporter::new();

    Keywords::validate(language, &mut reporter);
    Operators::validate(language, &mut reporter);
    Productions::validate(language, &mut reporter);
    Versions::validate(language, &mut reporter);

    reporter.into_result()
}
