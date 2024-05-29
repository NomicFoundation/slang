use std::rc::Rc;

use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::model::{
    EnumItem, FragmentItem, Identifier, KeywordItem, PrecedenceItem, RepeatedItem, SeparatedItem,
    StructItem, TokenItem, TriviaItem,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, EnumDiscriminants, ParseInputTokens, WriteOutputTokens)]
pub enum Item {
    Struct { item: Rc<StructItem> },
    Enum { item: Rc<EnumItem> },
    Repeated { item: Rc<RepeatedItem> },
    Separated { item: Rc<SeparatedItem> },
    Precedence { item: Rc<PrecedenceItem> },

    Trivia { item: Rc<TriviaItem> },
    Keyword { item: Rc<KeywordItem> },
    Token { item: Rc<TokenItem> },
    Fragment { item: Rc<FragmentItem> },
}

impl Item {
    pub fn name(&self) -> &Identifier {
        match self {
            Item::Struct { item } => &item.name,
            Item::Enum { item } => &item.name,
            Item::Repeated { item } => &item.name,
            Item::Separated { item } => &item.name,
            Item::Precedence { item } => &item.name,

            Item::Trivia { item } => &item.name,
            Item::Keyword { item } => &item.name,
            Item::Token { item } => &item.name,
            Item::Fragment { item } => &item.name,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Item::Trivia { .. } | Item::Keyword { .. } | Item::Token { .. } | Item::Fragment { .. }
        )
    }

    pub fn is_nonterminal(&self) -> bool {
        // Items are disjoint, hence it's enough to negate this.
        !self.is_terminal()
    }
}
