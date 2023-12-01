use crate::model::{
    EnumItem, FragmentItem, Identifier, KeywordItem, PrecedenceItem, RepeatedItem, SeparatedItem,
    StructItem, TokenItem, TriviaItem,
};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum Item {
    Struct { item: StructItem },
    Enum { item: EnumItem },
    Repeated { item: RepeatedItem },
    Separated { item: SeparatedItem },
    Precedence { item: PrecedenceItem },

    Trivia { item: TriviaItem },
    Keyword { item: KeywordItem },
    Token { item: TokenItem },
    Fragment { item: FragmentItem },
}

impl Item {
    pub fn name(&self) -> &Identifier {
        return match self {
            Item::Struct { item } => &item.name,
            Item::Enum { item } => &item.name,
            Item::Repeated { item } => &item.name,
            Item::Separated { item } => &item.name,
            Item::Precedence { item } => &item.name,

            Item::Trivia { item } => &item.name,
            Item::Keyword { item } => &item.name,
            Item::Token { item } => &item.name,
            Item::Fragment { item } => &item.name,
        };
    }
}
