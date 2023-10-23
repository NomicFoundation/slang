mod derive;
mod model;
mod utils;

use crate::{model::Model, utils::error};
use quote::{quote, ToTokens};
use std::borrow::BorrowMut;
use syn::Result;

#[proc_macro_attribute]
pub fn derive_internals(
    config: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return match derive_internals_aux(config, input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    };
}

fn derive_internals_aux(
    config: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    if !config.is_empty() {
        return error(
            proc_macro2::TokenStream::from(config),
            "Attribute does not support configuration.",
        );
    }

    let mut input_mod = syn::parse::<syn::ItemMod>(input)?;

    let input_items = match input_mod.content.borrow_mut() {
        Some((_, items)) => items,
        None => return error(input_mod, "Expected a module containing items within."),
    };

    // Use [syn::Item::Verbatim] so that we don't end up parsing the generated items:
    let output = run_derivers(input_items)?;
    input_items.push(syn::Item::Verbatim(output));

    return Ok(input_mod.into_token_stream().into());
}

fn run_derivers(input_items: &Vec<syn::Item>) -> Result<proc_macro2::TokenStream> {
    let model = Model::from_syn(input_items)?;

    let spanned = model.items().map(derive::spanned);
    let parse_input_tokens = model.items().map(derive::parse_input_tokens);
    let write_output_tokens = model.items().map(derive::write_output_tokens);

    // Group macro-generated items in a 'spanned' sub-module.
    // It is visible only to the definition crate, and never leaked outside the context of the macro.
    // Only the parent types are returned to the user via the generated definition.
    return Ok(quote! {
        pub(crate) mod spanned {
            pub use super::*;

            #(#spanned)*
            #(#parse_input_tokens)*
            #(#write_output_tokens)*
        }
    });
}
