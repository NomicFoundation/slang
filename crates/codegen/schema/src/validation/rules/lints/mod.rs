mod children_count;
mod consistent_shape;

use codegen_utils::errors::CodegenResult;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::lints::{children_count::ChildrenCount, consistent_shape::ConsistentShape},
        visitors::Reporter,
    },
};

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    ChildrenCount::validate(language, &mut reporter);
    ConsistentShape::validate(language, &mut reporter);

    return reporter.to_result();
}
