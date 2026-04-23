use language_v2_internal_macros::WriteOutputTokens;
use proc_macro2::{Delimiter, Group, TokenStream};
use serde::{Deserialize, Serialize};
use syn::parse::ParseStream;

use crate::internals::{Error, ErrorsCollection, ParseHelpers, ParseInputTokens, Result};

/// A wrapper type for embedding free-form code inside the DSL.
///
/// Syntactically, `Code(...)` is just another DSL node — the same shape as
/// `Struct(...)`, `Topic(...)`, or `ParserOptions(...)` — but instead of parsing its
/// body as structured fields, the DSL parser captures every token inside the parens
/// verbatim and stores the result as a `String`.
///
/// The body must tokenize as valid Rust (balanced delimiters, etc.), but is otherwise
/// free-form. Comments are stripped by the Rust lexer before the proc-macro sees them,
/// so they are not preserved in the captured string.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, WriteOutputTokens,
)]
#[serde(transparent)]
pub struct Code {
    pub value: String,
}

impl ParseInputTokens for Code {
    /// `Code` must always be invoked as a named value (`Code(...)`), handled by
    /// [`Self::parse_named_value`]. This direct path is unsupported: it would
    /// route the body through a nested `ParseStream`, which rebuilds the
    /// `TokenStream` and loses the original
    /// source-level formatting — turning `Foo<T>` into `Foo < T >`, which
    /// LALRPOP rejects when the captured string is embedded into a generated
    /// `.lalrpop` grammar.
    ///
    /// Note: If you find a valid reason to support it, do it and comment the reason.
    fn parse_value(input: ParseStream<'_>, _: &mut ErrorsCollection) -> Result<Self> {
        let tokens: TokenStream = ParseHelpers::syn(input)?;
        Error::fatal(&tokens, &Errors::UnsupportedDirectCall)
    }

    /// Parse `Code(...)` at a named-value position and capture the parenthesized
    /// body verbatim.
    ///
    /// ## Why we parse a `Group` directly instead of using `ParseHelpers::delimited`
    ///
    /// `ParseHelpers::delimited` descends into the body via syn's `parenthesized!`,
    /// which creates a nested `ParseStream` backed by a fresh `TokenBuffer` over
    /// the group's contents. Pulling a `TokenStream` back out of that buffer
    /// rebuilds a new stream token-by-token; the reconstructed stream no longer
    /// has the compiler-side source-level backing, and its `Display` falls back
    /// to per-token `Spacing`-based formatting — not sufficient to reproduce
    /// `Foo<T>`.
    ///
    /// Grabbing the body as a `proc_macro2::Group` and calling `group.stream()`
    /// returns the original `TokenStream` handle untouched, so `to_string()`
    /// produces `Foo<T>` exactly as written — what LALRPOP expects.
    fn parse_named_value(input: ParseStream<'_>, _: &mut ErrorsCollection) -> Result<Self> {
        let name = ParseHelpers::syn::<syn::Ident>(input)?;
        if name != "Code" {
            return Error::fatal(&name, &Errors::ExpectedCode);
        }

        let group = ParseHelpers::syn::<Group>(input)?;
        if group.delimiter() != Delimiter::Parenthesis {
            return Error::fatal(&group, &Errors::ExpectedParens);
        }

        Ok(Self {
            value: group.stream().to_string(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Expected `Code(...)` DSL node.")]
    ExpectedCode,
    #[error("`Code` body must be delimited with `(...)`.")]
    ExpectedParens,
    #[error("`Code` must be parsed via `parse_named_value` to preserve source-level formatting.")]
    UnsupportedDirectCall,
}
