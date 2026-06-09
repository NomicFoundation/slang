use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::Identifier;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum EvmTargetSpecifier {
    #[default]
    Always,
    From {
        from: Identifier,
    },
    Till {
        till: Identifier,
    },
    Range {
        from: Identifier,
        till: Identifier,
    },
}
