mod collector;
mod metadata;
mod validator;

use codegen_utils::errors::CodegenResult;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::references::{collector::Collector, validator::Validator},
        visitors::{run_visitor, Reporter},
    },
};

pub fn run(language: &LanguageDefinitionRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    let metadata = {
        let mut collector = Collector::new();
        run_visitor(&mut collector, language, &mut reporter);

        collector.metadata()
    };

    let metadata = {
        let mut validator = Validator::new(language, metadata);
        run_visitor(&mut validator, language, &mut reporter);

        validator.metadata()
    };

    metadata.check_not_used(language, &mut reporter);
    metadata.check_not_reachable(language, &mut reporter);

    return reporter.to_result();
}
