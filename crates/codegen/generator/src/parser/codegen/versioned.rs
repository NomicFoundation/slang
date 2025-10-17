use language_definition::model::VersionSpecifier;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use semver::Version;

use crate::parser::grammar::{Labeled, ParserDefinitionNode};

pub(super) trait Versioned {
    fn version_specifier(&self) -> Option<&VersionSpecifier>;
}

impl Versioned for ParserDefinitionNode {
    fn version_specifier(&self) -> Option<&VersionSpecifier> {
        match self {
            ParserDefinitionNode::Versioned(_, version_specifier) => Some(version_specifier),

            ParserDefinitionNode::Optional(value)
            | ParserDefinitionNode::ZeroOrMore(Labeled { value, .. })
            | ParserDefinitionNode::OneOrMore(Labeled { value, .. }) => value.version_specifier(),

            _ => None,
        }
    }
}

pub(super) trait VersionedQuote {
    /// Depending on the `as_bool_expr` result, wraps the given code in an `if` block and optionally includes an `else` block
    fn to_conditional_code(
        &self,
        if_true: TokenStream,
        if_false: Option<TokenStream>,
    ) -> TokenStream;
    /// Quotes a boolean expression that is satisfied for the given version quality ranges
    fn as_bool_expr(&self) -> TokenStream;
}

impl VersionedQuote for Option<&VersionSpecifier> {
    fn to_conditional_code(
        &self,
        if_true: TokenStream,
        if_false: Option<TokenStream>,
    ) -> TokenStream {
        if self.is_none() {
            if_true
        } else {
            let condition = self.as_bool_expr();

            let else_part = if_false.map(|if_false| quote! { else { #if_false } });
            quote! { if #condition { #if_true } #else_part }
        }
    }

    fn as_bool_expr(&self) -> TokenStream {
        let to_version_flag_name = |v: &Version| {
            format_ident!(
                "version_is_at_least_{v}",
                v = &v.to_string().replace('.', "_")
            )
        };

        match self {
            // No constraints imposed, so always enabled
            None => quote!(true),
            Some(VersionSpecifier::Never) => quote!(false),
            Some(VersionSpecifier::From { from }) => {
                let flag = to_version_flag_name(from);
                quote! { self.#flag }
            }
            Some(VersionSpecifier::Till { till }) => {
                let flag = to_version_flag_name(till);
                quote! { ! self.#flag }
            }
            Some(VersionSpecifier::Range { from, till }) => {
                let from_flag = to_version_flag_name(from);
                let till_flag = to_version_flag_name(till);
                quote! { self.#from_flag && ! self.#till_flag }
            }
        }
    }
}
