use crate::model::Identifier;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub enum Scanner {
    Sequence {
        scanners: Vec<Scanner>,
    },
    Choice {
        scanners: Vec<Scanner>,
    },
    Optional {
        scanner: Box<Scanner>,
    },
    ZeroOrMore {
        scanner: Box<Scanner>,
    },
    OneOrMore {
        scanner: Box<Scanner>,
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
    TrailingContext {
        scanner: Box<Scanner>,
        not_followed_by: Box<Scanner>,
    },
    Fragment {
        reference: Identifier,
    },
}
