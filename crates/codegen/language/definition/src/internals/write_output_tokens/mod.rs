mod implementations;

pub trait WriteOutputTokens {
    fn write_output_tokens(&self) -> proc_macro2::TokenStream;
}
