use crate::parse_input::{InputField, InputItem, InputVariant};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn write_output_tokens(item: &InputItem) -> TokenStream {
    let (name, implementation) = match item {
        InputItem::Struct { name, fields, .. } => (name, derive_struct(name, fields)),
        InputItem::Enum { name, variants, .. } => (name, derive_enum(name, variants)),
    };

    return quote! {
        impl crate::internals::WriteOutputTokens for #name {
            fn write_output_tokens(&self) -> proc_macro2::TokenStream {
                #implementation
            }
        }
    };
}

fn derive_struct(name: &Ident, fields: &Vec<InputField>) -> TokenStream {
    let keys = fields.iter().map(|field| &field.name).collect_vec();

    return quote! {
        #( let #keys = self.#keys.write_output_tokens(); )*

        return quote::quote! {
            codegen_language_definition::model::#name {
                #( #keys: ##keys ),*
            }
        };
    };
}

fn derive_enum(name: &Ident, variants: &Vec<InputVariant>) -> TokenStream {
    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.name;

        match &variant.fields {
            None => {
                return quote! {
                    Self::#variant_name => {
                        return quote::quote! {
                            codegen_language_definition::model::#name::#variant_name
                        };
                    }
                };
            }
            Some(fields) => {
                let keys = fields.iter().map(|field| &field.name).collect_vec();

                return quote! {
                    Self::#variant_name {
                        #( #keys ),*
                    } => {
                        #( let #keys = #keys.write_output_tokens(); )*

                        return quote::quote! {
                            codegen_language_definition::model::#name::#variant_name {
                                #( #keys: ##keys ),*
                            }
                        };
                    }
                };
            }
        };
    });

    return quote! {
        match self {
            #( #match_arms )*
        };
    };
}
