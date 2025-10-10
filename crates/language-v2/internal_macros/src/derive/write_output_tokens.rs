use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::input_model::{strip_spanned_prefix, InputField, InputItem, InputVariant};

pub fn write_output_tokens(item: InputItem) -> TokenStream {
    match item {
        InputItem::Struct { name, fields } => derive_struct(&name, &fields),
        InputItem::Enum { name, variants } => derive_enum(&name, &variants),
    }
}

fn derive_struct(name: &Ident, fields: &[InputField]) -> TokenStream {
    let stripped_name = format_ident!("{}", strip_spanned_prefix(name.to_string()));

    let keys = fields.iter().map(|field| &field.name).collect_vec();

    quote! {
        impl crate::internals::WriteOutputTokens for #name {
            fn write_output_tokens(&self) -> proc_macro2::TokenStream {
                #( let #keys = self.#keys.write_output_tokens(); )*

                return quote::quote! {
                    language_v2_definition::model::#stripped_name {
                        #( #keys: ##keys ),*
                    }
                };
            }
        }
    }
}

fn derive_enum(name: &Ident, variants: &[InputVariant]) -> TokenStream {
    let stripped_name = format_ident!("{}", strip_spanned_prefix(name.to_string()));

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.name;

        match &variant.fields {
            None => {
                quote! {
                    Self::#variant_name => {
                        return quote::quote! {
                            language_v2_definition::model::#stripped_name::#variant_name
                        };
                    }
                }
            }
            Some(fields) => {
                let keys = fields.iter().map(|field| &field.name).collect_vec();

                quote! {
                    Self::#variant_name {
                        #( #keys ),*
                    } => {
                        #( let #keys = #keys.write_output_tokens(); )*

                        return quote::quote! {
                            language_v2_definition::model::#stripped_name::#variant_name {
                                #( #keys: ##keys ),*
                            }
                        };
                    }
                }
            }
        }
    });

    quote! {
        impl crate::internals::WriteOutputTokens for #name {
            fn write_output_tokens(&self) -> proc_macro2::TokenStream {
                match self {
                    #( #match_arms )*
                };
            }
        }
    }
}
