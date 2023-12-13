mod children_count;
mod consistent_shape;

use anyhow::Result;

use crate::types::LanguageDefinitionRef;
use crate::validation::rules::lints::children_count::ChildrenCount;
use crate::validation::rules::lints::consistent_shape::ConsistentShape;
use crate::validation::visitors::Reporter;

pub fn run(language: &LanguageDefinitionRef) -> Result<()> {
    let mut reporter = Reporter::new();

    ChildrenCount::validate(language, &mut reporter);
    ConsistentShape::validate(language, &mut reporter);

    reporter.into_result()
}
