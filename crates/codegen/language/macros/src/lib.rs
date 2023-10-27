use codegen_language_definition::compiler::LanguageCompiler;
use proc_macro::TokenStream;

#[proc_macro]
pub fn compile(input: TokenStream) -> TokenStream {
    return LanguageCompiler::compile(input.into()).into();
}
