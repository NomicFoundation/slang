mod derive;
mod input_model;

use proc_macro::TokenStream;
use quote::ToTokens;

use crate::input_model::InputItem;

#[proc_macro_attribute]
pub fn derive_spanned_type(args: TokenStream, mut input: TokenStream) -> TokenStream {
    let spanned_type = {
        let spanned_derive_args = args.into();
        let input = input.clone();

        let item = syn::parse_macro_input!(input as InputItem);
        derive::spanned(item, spanned_derive_args)
    };

    input.extend(TokenStream::from(spanned_type.into_token_stream()));
    input
}

#[proc_macro_derive(ParseInputTokens)]
pub fn derive_parse_input_tokens(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as InputItem);

    derive::parse_input_tokens(item).into()
}

#[proc_macro_derive(WriteOutputTokens)]
pub fn derive_write_output_tokens(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as InputItem);

    derive::write_output_tokens(item).into()
}
