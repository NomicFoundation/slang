mod collector;
mod metadata;
mod validator;

use codegen_utils::errors::CodegenResult;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::references::{collector::Collector, metadata::Metadata, validator::Validator},
        visitors::Reporter,
    },
};

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut metadata = Metadata::new();
    let mut reporter = Reporter::new();

    Collector::collect(language, &mut metadata, &mut reporter);
    Validator::validate(language, &mut metadata, &mut reporter);

    metadata.validate_not_used(language, &mut reporter);
    metadata.validate_not_reachable(language, &mut reporter);

    return reporter.to_result();
}
