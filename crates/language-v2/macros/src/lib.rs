use language_v2_definition::compiler::LanguageCompiler;
use proc_macro::TokenStream;

#[proc_macro]
pub fn compile(input: TokenStream) -> TokenStream {
    LanguageCompiler::compile(input.into()).into()
}
