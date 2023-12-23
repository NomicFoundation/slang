use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::model::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
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
