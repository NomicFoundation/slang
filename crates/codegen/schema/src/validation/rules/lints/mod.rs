mod children_count;
mod consistent_shape;

use anyhow::Result;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::lints::{children_count::ChildrenCount, consistent_shape::ConsistentShape},
        visitors::Reporter,
    },
};

pub fn run(language: &LanguageDefinitionRef) -> Result<()> {
    let mut reporter = Reporter::new();

    ChildrenCount::validate(language, &mut reporter);
    ConsistentShape::validate(language, &mut reporter);

    reporter.into_result()
}
