use indexmap::IndexMap;
use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};

/// A `StructItem` is a nonterminal that can have fields.
/// It roughly corresponds to a sequend of `Item`s.
///
/// It generates:
/// - A nonterminal named `name` with a single field for each field in `fields`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct StructItem {
    pub name: Identifier,

    /// Whether the struct is enabled
    ///
    /// Defaults to `Always`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// Whether error recovery is an option for this struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_recovery: Option<FieldsErrorRecovery>,

    /// The fields of the struct, in the order they should appear in the source code
    #[serde(with = "indexmap::map::serde_seq")]
    pub fields: IndexMap<Identifier, Field>,
}
