mod analysis;
mod emitter;
mod versions;

use crate::{
    compiler::{analysis::Analysis, emitter::LanguageEmitter},
    internals::ParseAdapter,
};
use proc_macro2::TokenStream;

pub struct LanguageCompiler;

impl LanguageCompiler {
    pub fn compile(input: TokenStream) -> TokenStream {
        let parse_output = match ParseAdapter::parse(input) {
            // Parsed all input successfully, with potentionally recoverable errors:
            Ok(parse_output) => parse_output,
            // Non-recoverable parsing error. Let's quit early:
            Err(error) => return error.to_compile_error(),
        };

        let analysis = Analysis::analyze(parse_output);

        let output = LanguageEmitter::emit(&analysis);

        return output;
    }
}
