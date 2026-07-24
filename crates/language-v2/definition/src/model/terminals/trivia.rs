use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, TokenScanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Identifier,

    pub scanner: TokenScanner,
}
