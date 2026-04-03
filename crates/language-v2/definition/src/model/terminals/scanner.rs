use indexmap::IndexSet;
use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum TokenScanner {
    Sequence {
        scanners: Vec<TokenScanner>,
    },
    Choice {
        scanners: Vec<TokenScanner>,
    },
    Optional {
        scanner: Box<TokenScanner>,
    },
    ZeroOrMore {
        scanner: Box<TokenScanner>,
    },
    OneOrMore {
        scanner: Box<TokenScanner>,
    },
    Not {
        chars: IndexSet<char>,
    },
    Range {
        inclusive_start: char,
        inclusive_end: char,
    },
    Atom {
        atom: String,
    },
    Fragment {
        reference: Identifier,
    },
}

impl TokenScanner {
    pub fn is_unique(&self) -> bool {
        match self {
            Self::Sequence { scanners } => scanners.iter().all(|scanner| scanner.is_unique()),
            Self::Atom { .. } => true,
            Self::Choice { .. }
            | Self::Optional { .. }
            | Self::ZeroOrMore { .. }
            | Self::OneOrMore { .. }
            | Self::Not { .. }
            | Self::Range { .. }
            | Self::Fragment { .. } => false,
        }
    }
}
