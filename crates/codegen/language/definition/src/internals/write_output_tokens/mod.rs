mod external_types;

pub trait WriteOutputTokens {
    fn write_output_tokens(&self) -> proc_macro2::TokenStream;
}
