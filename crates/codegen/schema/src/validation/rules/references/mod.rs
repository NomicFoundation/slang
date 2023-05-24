mod collector;
mod metadata;
mod validator;

use codegen_utils::errors::CodegenResult;

use crate::{
    types::SchemaRef,
    validation::{
        rules::references::{collector::Collector, validator::Validator},
        visitors::{run_visitor, Reporter},
    },
};

pub fn run(schema: &SchemaRef) -> CodegenResult<()> {
    let mut reporter = Reporter::new();

    let metadata = {
        let mut collector = Collector::new();
        run_visitor(&mut collector, schema, &mut reporter);

        collector.metadata()
    };

    let metadata = {
        let mut validator = Validator::new(schema, metadata);
        run_visitor(&mut validator, schema, &mut reporter);

        validator.metadata()
    };

    metadata.check_not_used(schema, &mut reporter);
    metadata.check_not_reachable(schema, &mut reporter);

    return reporter.to_result();
}
