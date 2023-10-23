use crate::model::{Field, Item, Variant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{fold::Fold, parse_quote};

pub fn spanned(item: &Item) -> TokenStream {
    match item {
        Item::Struct {
            name,
            fields,
            attributes,
        } => {
            let fields = fields.iter().map(derive_field);

            return quote! {
                #( #attributes )*
                pub struct #name {
                    #( pub #fields ),*
                }
            };
        }
        Item::Enum {
            name,
            variants,
            attributes,
        } => {
            let variants = variants.iter().map(derive_variant);

            return quote! {
                #( #attributes )*
                pub enum #name {
                    #(#variants),*
                }
            };
        }
    };
}

fn derive_variant(variant: &Variant) -> TokenStream {
    let name = &variant.name;

    match &variant.fields {
        None => {
            return quote! {
                #name
            };
        }
        Some(fields) => {
            let fields = fields.iter().map(derive_field);

            return quote! {
                #name {
                    #( #fields ),*
                }
            };
        }
    };
}

fn derive_field(field: &Field) -> TokenStream {
    let name = &field.name;
    let tipe = &field.tipe;

    let tipe = match RewriteGenericArgs::rewrite(tipe.clone()) {
        Some(tipe) => tipe,
        None => {
            // If there are no generic args, wrap the top-level type itself:
            parse_quote! {
                crate::internals::Spanned<#tipe>
            }
        }
    };

    return quote! {
        #name: #tipe
    };
}

struct RewriteGenericArgs {
    found_generic_args: bool,
}

impl RewriteGenericArgs {
    fn rewrite(input: syn::Type) -> Option<syn::Type> {
        let mut instance = RewriteGenericArgs {
            found_generic_args: false,
        };

        let result = instance.fold_type(input);

        return if instance.found_generic_args {
            Some(result)
        } else {
            None
        };
    }
}

impl Fold for RewriteGenericArgs {
    fn fold_generic_argument(&mut self, input: syn::GenericArgument) -> syn::GenericArgument {
        if let syn::GenericArgument::Type(inner) = input {
            self.found_generic_args = true;

            let result = match RewriteGenericArgs::rewrite(inner.clone()) {
                Some(inner) => inner,
                None => {
                    // If the inner type was not already wrapped, wrap this one instead:
                    parse_quote! {
                        crate::internals::Spanned<#inner>
                    }
                }
            };

            return syn::GenericArgument::Type(result);
        } else {
            return syn::fold::fold_generic_argument(self, input);
        }
    }
}
