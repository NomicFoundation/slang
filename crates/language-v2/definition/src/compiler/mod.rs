mod analysis;
mod emitter;
mod utils;

use proc_macro2::TokenStream;

use crate::compiler::analysis::Analysis;
use crate::compiler::emitter::LanguageEmitter;
use crate::internals::ParseAdapter;

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

        LanguageEmitter::emit(&analysis)
    }
}
