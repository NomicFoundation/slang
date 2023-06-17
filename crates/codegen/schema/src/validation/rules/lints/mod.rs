mod children_count;
mod empty_roots;

use codegen_utils::errors::CodegenResult;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::lints::{children_count::ChildrenCount, empty_roots::EmptyRoots},
        visitors::Reporter,
    },
};

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    ChildrenCount::check(language, &mut reporter);
    EmptyRoots::check(language, &mut reporter);

    return reporter.to_result();
}
