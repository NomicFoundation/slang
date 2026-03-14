use indexmap::IndexMap;
use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Field, Identifier, ParserOptions, VersionSpecifier};

/// A `StructItem` is a nonterminal that can have fields.
/// It roughly corresponds to a sequence of `Item`s.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct StructItem {
    pub name: Identifier,

    /// Whether the struct is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// If set, this struct acts as a "gateway" that switches to a different lexical context.
    /// The first field references a keyword in the struct's own context, while remaining fields
    /// reference items in the specified target context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_lexical_context: Option<Identifier>,

    /// The fields of the struct, in the order they should appear in the source code
    #[serde(with = "indexmap::map::serde_seq")]
    pub fields: IndexMap<Identifier, Field>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser_options: Option<ParserOptions>,
}
