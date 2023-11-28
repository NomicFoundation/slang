use crate::model::{Field, Item, Variant};
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

pub fn parse_input_tokens(item: &Item) -> TokenStream {
    return match item {
        Item::Struct { name, fields, .. } => derive_struct(name, fields),
        Item::Enum { name, variants, .. } => derive_enum(name, variants),
    };
}

fn derive_struct(name: &Ident, fields: &[Field]) -> TokenStream {
    let name_string = Literal::string(&name.to_string());
    let unexpected_type_error = Literal::string(&format!("Expected type: {name}"));

    let fields_return = derive_fields_return(quote!(Self), fields);

    return quote! {
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
                if name != #name_string {
                    return crate::internals::Error::fatal(&name, &#unexpected_type_error);
                }

                return crate::internals::ParseHelpers::delimited(
                    proc_macro2::Delimiter::Parenthesis,
                    input,
                    |_, input| Self::parse_value(&input, errors)
                );
            }
        }
    };
}

fn derive_enum(name: &Ident, variants: &[Variant]) -> TokenStream {
    let match_arms = variants.iter().map(|variant| {
        let variant_id = &variant.name;
        let variant_name = variant_id.to_string();

        if let Some(fields) = &variant.fields {
            let fields_return = derive_fields_return(quote!(Self::#variant_id), fields);

            return quote! {
                #variant_name => {
                    return crate::internals::ParseHelpers::delimited(
                        proc_macro2::Delimiter::Parenthesis,
                        input,
                        |_, input| { #fields_return }
                    );
                }
            };
        } else {
            return quote! {
                #variant_name => {
                    return Ok(Self::#variant_id);
                }
            };
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

    return quote! {
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
    };
}

fn derive_fields_return(type_name: TokenStream, fields: &[Field]) -> TokenStream {
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

            return quote!(
                #name: crate::internals::ParseInputTokens::parse_field(#name_string, &input, errors)?,
            );
        }).collect()
    };

    return quote! {
        return Ok(#type_name {
            #assignments
        });
    };
}
