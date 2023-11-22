mod derive;
mod parse_input;

use crate::parse_input::InputItem;
use proc_macro::TokenStream;

#[proc_macro_derive(ParseInputTokens)]
pub fn parse_input_tokens(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as InputItem);

    return derive::parse_input_tokens(&item).into();
}

#[proc_macro_derive(WriteOutputTokens)]
pub fn write_output_tokens(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as InputItem);

    return derive::write_output_tokens(&item).into();
}
