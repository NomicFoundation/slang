use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

use crate::input_model::{strip_spanned_prefix, InputField, InputItem, InputVariant};

pub fn parse_input_tokens(item: InputItem) -> TokenStream {
    match item {
        InputItem::Struct { name, fields } => derive_struct(&name, &fields),
        InputItem::Enum { name, variants } => derive_enum(&name, &variants),
    }
}

fn derive_struct(name: &Ident, fields: &[InputField]) -> TokenStream {
    let stripped_name = Literal::string(&strip_spanned_prefix(name.to_string()));
    let unexpected_type_error = Literal::string(&format!("Expected type: {stripped_name}"));

    let fields_return = derive_fields_return(&quote!(Self), fields);

    quote! {
        impl crate::internals::ParseInputTokens for #name {
            fn parse_value(
                input: syn::parse::ParseStream,
                errors: &mut crate::internals::ErrorsCollection,
            ) -> crate::internals::Result<Self> {
                #fields_return
            }

            fn parse_named_value(
                input: syn::parse::ParseStream,
                errors: &mut crate::internals::ErrorsCollection,
            ) -> crate::internals::Result<Self> {
                let name = crate::internals::ParseHelpers::syn::<syn::Ident>(input)?;
                if name != #stripped_name {
                    return crate::internals::Error::fatal(&name, &#unexpected_type_error);
                }

                return crate::internals::ParseHelpers::delimited(
                    proc_macro2::Delimiter::Parenthesis,
                    input,
                    |_, input| Self::parse_value(&input, errors)
                );
            }
        }
    }
}

fn derive_enum(name: &Ident, variants: &[InputVariant]) -> TokenStream {
    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.name;
        let variant_name = variant_ident.to_string();

        if let Some(fields) = &variant.fields {
            let fields_return = derive_fields_return(&quote!(Self::#variant_ident), fields);

            quote! {
                #variant_name => {
                    return crate::internals::ParseHelpers::delimited(
                        proc_macro2::Delimiter::Parenthesis,
                        input,
                        |_, input| { #fields_return }
                    );
                }
            }
        } else {
            quote! {
                #variant_name => {
                    return Ok(Self::#variant_ident);
                }
            }
        }
    });

    let unknown_variant_error = Literal::string(&format!(
        "Expected a variant: {}",
        variants
            .iter()
            .map(|variant| format!("'{}'", variant.name))
            .collect_vec()
            .join(" or ")
    ));

    quote! {
        impl crate::internals::ParseInputTokens for #name {
            fn parse_value(
                input: syn::parse::ParseStream,
                errors: &mut crate::internals::ErrorsCollection,
            ) -> crate::internals::Result<Self> {
                let variant = crate::internals::ParseHelpers::syn::<syn::Ident>(input)?;
                match variant.to_string().as_str() {
                    #( #match_arms )*

                    _ => {
                        return crate::internals::Error::fatal(&variant, &#unknown_variant_error);
                    }
                };
            }
        }
    }
}

fn derive_fields_return(type_name: &TokenStream, fields: &[InputField]) -> TokenStream {
    // When there is only one field, we omit the `key = ` part.
    // This way, we can just write `Foo(Bar)` instead of `Foo(key = Bar)`.
    let assignments = if let [single_field] = fields {
        let name = &single_field.name;
        quote!(
            #name: crate::internals::ParseInputTokens::parse_value(&input, errors)?
        )
    } else {
        fields.iter().map(|field| {
            let name = &field.name;
            let name_string = name.to_string();

            quote!(
                #name: crate::internals::ParseInputTokens::parse_field(#name_string, &input, errors)?,
            )
        }).collect()
    };

    quote! {
        return Ok(#type_name {
            #assignments
        });
    }
}
