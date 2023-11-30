use crate::input_model::{add_spanned_prefix, InputField, InputItem, InputVariant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{fold::Fold, parse_quote, Error, GenericArgument, Type};

pub fn spanned(item: InputItem, spanned_derive_args: TokenStream) -> TokenStream {
    let derive_attribute = if spanned_derive_args.is_empty() {
        spanned_derive_args
    } else {
        quote! {
            #[derive(#spanned_derive_args)]
        }
    };

    match item {
        InputItem::Struct { name, fields } => {
            let name = format_ident!("{}", add_spanned_prefix(name.to_string()));
            let fields = fields.into_iter().map(derive_field);

            return quote! {
                #derive_attribute
                pub(crate) struct #name {
                    #( pub #fields ),*
                }
            };
        }
        InputItem::Enum { name, variants } => {
            let name = format_ident!("{}", add_spanned_prefix(name.to_string()));
            let variants = variants.into_iter().map(derive_variant);

            return quote! {
                #derive_attribute
                pub(crate) enum #name {
                    #(#variants),*
                }
            };
        }
    };
}

fn derive_variant(variant: InputVariant) -> TokenStream {
    let name = &variant.name;

    match variant.fields {
        None => {
            return quote! {
                #name
            };
        }
        Some(fields) => {
            let fields = fields.into_iter().map(derive_field);

            return quote! {
                #name {
                    #( #fields ),*
                }
            };
        }
    };
}

fn derive_field(field: InputField) -> TokenStream {
    let InputField { name, r#type } = field;
    let r#type = RewriteFieldType::execute(r#type);

    return quote! {
        #name: #r#type
    };
}

struct RewriteFieldType {
    has_generic_args: bool,
}

impl RewriteFieldType {
    fn execute(input: Type) -> Type {
        let mut instance = RewriteFieldType {
            has_generic_args: false,
        };

        let result = instance.fold_type(input);

        if instance.has_generic_args {
            // Return generic types as-is. Inner arguments have been already wrapped:
            return result;
        }

        // Return the spanned type:
        return get_spanned_type(result);
    }
}

impl Fold for RewriteFieldType {
    fn fold_generic_argument(&mut self, input: GenericArgument) -> GenericArgument {
        if let GenericArgument::Type(inner) = input {
            self.has_generic_args = true;

            // Proceed to wrap the generic arg, using a new visitor:
            return GenericArgument::Type(RewriteFieldType::execute(inner));
        } else {
            return syn::fold::fold_generic_argument(self, input);
        }
    }
}

fn get_spanned_type(input: Type) -> Type {
    let type_name = input.to_token_stream().to_string();
    match type_name.as_str() {
        // These are model Types that have a derived 'SpannedXXX' type.
        // Let's use that instead:
        "EnumItem"
        | "EnumVariant"
        | "Field"
        | "FieldDelimiters"
        | "FieldKind"
        | "FieldsErrorRecovery"
        | "FragmentItem"
        | "InputItem"
        | "Item"
        | "KeywordDefinition"
        | "KeywordItem"
        | "KeywordValue"
        | "PrecedenceExpression"
        | "PrecedenceItem"
        | "PrecedenceOperator"
        | "PrimaryExpression"
        | "RepeatedItem"
        | "Scanner"
        | "Section"
        | "SeparatedItem"
        | "StructItem"
        | "TokenDefinition"
        | "TokenItem"
        | "Topic"
        | "TriviaItem"
        | "TriviaParser" => {
            let spanned_type = format_ident!("{}", add_spanned_prefix(type_name));
            return parse_quote! {
                crate::model::#spanned_type
            };
        }

        // These are model Types that have a derived 'SpannedXXX' type.
        // Let's use that instead, but also wrap it in 'Spanned<T>' because we want to capture its complete span for validation:
        "OperatorModel" | "VersionSpecifier" => {
            let spanned_type = format_ident!("{}", add_spanned_prefix(type_name));
            return parse_quote! {
                crate::internals::Spanned<crate::model::#spanned_type>
            };
        }

        // These are model Types that don't have a derived 'SpannedXXX' type.
        // Let's just wrap it in 'Spanned<T>':
        "Identifier" => {
            return parse_quote! {
                crate::internals::Spanned<#input>
            };
        }

        // External types should also be wrapped in 'Spanned<T>':
        "bool" | "char" | "String" | "Version" => {
            return parse_quote! {
                crate::internals::Spanned<#input>
            };
        }

        _ => {
            let message = format!(
                "Unrecognized type '{type_name}'. Update the list of predefined model types in: {file}:{line}",
                file = file!(),
                line = line!(),
            );

            let error = Error::new_spanned(input, message).to_compile_error();
            return Type::Verbatim(error);
        }
    };
}
